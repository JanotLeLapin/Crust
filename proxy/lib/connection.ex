defmodule Proxy.Connection do
  use GenServer

  def start_link(options) do
    GenServer.start_link(__MODULE__, options, name: __MODULE__)
  end

  def send(pid, packet) do
    GenServer.cast(__MODULE__, {:send, pid, packet})
  end

  @impl true
  def init(_options) do
    {:ok, socket} = :gen_tcp.connect({127,0,0,1}, 8080, active: :once)
    {:ok, socket}
  end

  @impl true
  def handle_cast({:send, pid, packet}, socket) do
    # Received message from some client process, forward it to the server with pid
    message = [encode_pid(pid)] ++ packet
    socket |> :gen_tcp.send(message)
    {:noreply, socket}
  end

  @impl true
  def handle_info({:tcp, _, data}, socket) do
    # Received message from server, forward it to the requested process
    {pid, len} = decode_pid(data)
    pid |> GenServer.cast({:message, data |> Enum.drop(len)})

    socket |> :inet.setopts(active: :once)
    {:noreply, socket}
  end

  defp encode_pid(pid) do
    list = "#{inspect(pid)}" |> String.slice(5, 100) |> String.trim(">") |> :binary.bin_to_list()
    [length(list)] ++ list
  end

  defp decode_pid(data) do
    len = data |> Enum.at(0)
    pid = data |> Enum.slice(1, len)
    {IEx.Helpers.pid("#{pid}"), len + 1}
  end
end

