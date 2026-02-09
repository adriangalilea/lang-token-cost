defmodule RestApi.Router do
  use Plug.Router

  plug(RestApi.Middleware)
  plug(Plug.Parsers, parsers: [:json], json_decoder: Jason)
  plug(:match)
  plug(:dispatch)

  forward("/api/v1/users", to: RestApi.RoutesUsers)
  forward("/api/v1/posts", to: RestApi.RoutesPosts)

  get "/health" do
    send_json(conn, 200, %{status: "ok"})
  end

  match _ do
    send_json(conn, 404, %{detail: "Not found"})
  end

  def send_json(conn, status, body) do
    conn
    |> put_resp_header("content-type", "application/json")
    |> put_resp_header("access-control-allow-origin", "*")
    |> put_resp_header("access-control-allow-methods", "*")
    |> put_resp_header("access-control-allow-headers", "*")
    |> send_resp(status, Jason.encode!(body))
  end
end
