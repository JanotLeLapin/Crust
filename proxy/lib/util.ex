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

  defp write_varint(value, result) do
    if ((value &&& 0x80) == 0) do
      result ++ [value]
    else
      write_varint(value >>> 7, result ++ [(value &&& 0x7F)])
    end
  end

  def write_varint(value) do
    write_varint(value, [])
  end

  def int_to_bytes(value) do
    Enum.map(0..3, fn i ->
      value >>> (3 - i) * 8
    end)
  end
end

