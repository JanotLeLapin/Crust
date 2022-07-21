defmodule Proxy.Util do
  use Bitwise

  defp read_varint(idx, offset, res, data) do
    b = data |> Enum.at(idx)
    new = res ||| (b &&& 0x7F) <<< ((idx - offset) * 7)

    if b < 0x80 do
      {new, idx + 1}
    else
      read_varint(idx + 1, offset, new, data)
    end
  end

  def read_varint(data, offset) do
    read_varint(offset, offset, 0, data)
  end
end

