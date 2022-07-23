defmodule Proxy.Connection do
  use GenServer

  def start_link(options) do
    GenServer.start_link(__MODULE__, options, name: __MODULE__)
  end

  def send(pid, state, packet) do
    GenServer.cast(__MODULE__, {:send, pid, state, packet})
  end

  @impl true
  def init(_options) do
    {:ok, socket} = :gen_tcp.connect({127,0,0,1}, 8080, active: :once)
    {:ok, socket}
  end

  @impl true
  def handle_cast({:send, pid, state, packet}, socket) do
    # Received message from some client process, forward it to the server with pid
    message = JSON.encode!(%{
      pid: encode_pid(pid),
      state: state |> Map.delete("socket"),
      data: packet,
    }) |> :binary.bin_to_list()
    socket |> :gen_tcp.send((length(message) |> Proxy.Util.int_to_bytes()) ++ message)
    {:noreply, socket}
  end

  defp unmerge_packets(packets, result) do
    case length(packets) do
      0 -> result
      _ ->
        # Size is a 4 bytes integer
        size = packets |> Proxy.Util.bytes_to_int()
        unmerge_packets(packets |> Enum.drop(size + 4), result ++ [packets |> Enum.drop(4) |> Enum.take(size)])
    end
  end

  @impl true
  def handle_info({:tcp, _, packet}, socket) do
    # Received message from server, forward it to the requested process
    # Packets may be merged when sent at the same time, unmerge before decoding
    packet |> unmerge_packets([]) |> Enum.map(fn packet ->
      message = JSON.decode!(packet)
      pid = message["pid"] |> decode_pid()
      pid |> GenServer.cast({:message, message["data"]})

      if message |> Map.has_key?("state") do
        pid |> GenServer.cast({:state, message["state"]})
      end
    end)
    socket |> :inet.setopts(active: :once)
    {:noreply, socket}
  end

  defp encode_pid(pid) do
    "#{inspect(pid)}" |> String.slice(5, 100) |> String.trim(">")
  end

  defp decode_pid(pid) do
    IEx.Helpers.pid("#{pid}")
  end
end

