defmodule RestApi.RoutesUsers do
  use Plug.Router
  alias RestApi.{Models.User, Store, Router}

  plug :match
  plug :dispatch

  get "/" do
    conn = fetch_query_params(conn)
    page = parse_int(conn.query_params["page"], 1)
    per_page = parse_int(conn.query_params["per_page"], 20)
    search = conn.query_params["search"]
    result = Store.list_users(page, per_page, search)
    result = %{result | items: Enum.map(result.items, &User.to_json/1)}
    Router.send_json(conn, 200, result)
  end

  post "/" do
    user = Store.create_user(conn.body_params)
    Router.send_json(conn, 201, User.to_json(user))
  end

  get "/:id" do
    case Store.get_user(String.to_integer(id)) do
      nil -> Router.send_json(conn, 404, %{detail: "User not found"})
      user -> Router.send_json(conn, 200, User.to_json(user))
    end
  end

  patch "/:id" do
    case Store.update_user(String.to_integer(id), conn.body_params) do
      nil -> Router.send_json(conn, 404, %{detail: "User not found"})
      user -> Router.send_json(conn, 200, User.to_json(user))
    end
  end

  delete "/:id" do
    case Store.delete_user(String.to_integer(id)) do
      false -> Router.send_json(conn, 404, %{detail: "User not found"})
      true -> send_resp(conn, 204, "")
    end
  end

  defp parse_int(nil, default), do: default
  defp parse_int(val, _default), do: String.to_integer(val)
end
