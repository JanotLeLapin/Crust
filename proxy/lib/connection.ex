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
      state: state |> Map.delete(:socket),
      data: packet,
    })
    socket |> :gen_tcp.send(message)
    {:noreply, socket}
  end

  @impl true
  def handle_info({:tcp, _, packet}, socket) do
    # Received message from server, forward it to the requested process
    message = JSON.decode!(packet)
    message["pid"] |> decode_pid() |> GenServer.cast({:message, message["data"], message["state"]})
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

