defmodule RestApi.Middleware do
  @behaviour Plug
  require Logger

  @impl true
  def init(opts), do: opts

  @impl true
  def call(conn, _opts) do
    start = System.monotonic_time(:millisecond)

    Plug.Conn.register_before_send(conn, fn conn ->
      elapsed = System.monotonic_time(:millisecond) - start
      Logger.info("#{conn.method} #{conn.request_path} -> #{conn.status} (#{elapsed}ms)")
      conn
    end)
  end
end
