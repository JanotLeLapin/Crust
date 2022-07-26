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

  defp keep_alive(socket) do
    socket |> :inet.send([2, 0, 42])
    Process.sleep(10000)
    keep_alive(socket)
  end

  @impl true
  def handle_cast({:state, new_state}, state) do
    # Received state change from server process
    if new_state["state"] == 3 do
      # State is now playing, we may start ping process
      spawn_link(fn ->
        keep_alive(state["socket"])
      end)
    end
    # Update state
    {:noreply, new_state |> Map.put("socket", state["socket"])}
  end

  @impl true
  def handle_cast({:message, message}, state) do
    # Received message from server process, forward it to Minecraft client
    state["socket"] |> :gen_tcp.send(message)
    {:noreply, state}
  end

  @impl true
  def handle_info({:tcp, _, message}, state) do
    # Received message from Minecraft client

    state = case state["state"] do
      0 ->
        # Decode handshake packet directly from proxy
        # otherwise following packet (status request or login start)
        # will be sent with outdated state

        {packet_size, packet_size_length} = message |> Proxy.Util.read_varint(0)
        # Assume packet id is 0x00
        {_, offset} = message |> Proxy.Util.read_varint(packet_size_length)
        {protocol, offset} = message |> Proxy.Util.read_varint(offset)
        {address_size, offset} = message |> Proxy.Util.read_varint(offset)
        # Skip server address (string) and port number (short) and we get the next state
        next_state = message |> Enum.at(offset + address_size + 2)

        new_state = state |> Map.put("protocol", protocol) |> Map.put("state", next_state)

        # The following packet is sent right after the handshake packet, so the two
        # may be merged together.
        size = packet_size + packet_size_length
        if length(message) > size do
          Proxy.Connection.send(self(), new_state, message |> Enum.drop(size))
        end

        new_state
      _ ->
        Proxy.Connection.send(self(), state, message)
        state
    end

    state["socket"] |> :inet.setopts(active: :once)
    {:noreply, state}
  end

  @impl true
  def handle_info({:tcp_closed, _}, state) do
    {:noreply, state}
  end
end

