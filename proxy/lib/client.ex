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
    {:ok, %{
      "socket" => socket,
      # Connection state
      # 0 = handshaking
      # 1 = status
      # 2 = login
      # 3 = play
      "state" => 0,
    }}
  end

  @impl true
  def handle_cast({:message, message, new_state}, state) do
    # Received message from server process, forward it to Minecraft client
    state["socket"] |> :gen_tcp.send(message)
    {:noreply, new_state |> Map.put("socket", state["socket"])}
  end

  @impl true
  def handle_info({:tcp, _, message}, state) do
    # Received message from Minecraft client, forward it to server process
    Proxy.Connection.send(self(), state, message)
    state["socket"] |> :inet.setopts(active: :once)
    {:noreply, state}
  end

  @impl true
  def handle_info({:tcp_closed, _}, state) do
    {:noreply, state}
  end
end

