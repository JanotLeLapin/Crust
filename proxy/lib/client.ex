defmodule Proxy.Client do
  use GenServer
  require Logger

  def accept(port) do
    {:ok, listener} = :gen_tcp.listen(port, active: :once, reuseaddr: true)
    Logger.info("Listening on port #{port}")
    acceptor(listener)
  end

  defp acceptor(listener) do
    {:ok, socket} = listener |> :gen_tcp.accept()
    {:ok, pid} = GenServer.start(__MODULE__, socket)
    :ok = socket |> :gen_tcp.controlling_process(pid)
    acceptor(listener)
  end

  @impl true
  def init(socket) do
    {:ok, socket}
  end

  @impl true
  def handle_cast({:message, message}, socket) do
    # Received message from server process, forward it to Minecraft client
    socket |> :gen_tcp.send(message)
    {:noreply, socket}
  end

  @impl true
  def handle_info({:tcp, _, message}, socket) do
    # Received message from Minecraft client, forward it to server process
    Proxy.Connection.send(self(), message)
    socket |> :inet.setopts(active: :once)
    {:noreply, socket}
  end

  @impl true
  def handle_info({:tcp_closed, _}, socket) do
    {:noreply, socket}
  end
end

