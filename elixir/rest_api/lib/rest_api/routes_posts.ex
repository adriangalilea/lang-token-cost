defmodule RestApi.RoutesPosts do
  use Plug.Router
  alias RestApi.{Models.Post, Store, Router}

  plug(:match)
  plug(:dispatch)

  get "/" do
    conn = fetch_query_params(conn)
    page = parse_int(conn.query_params["page"], 1)
    per_page = parse_int(conn.query_params["per_page"], 20)
    author_id = parse_optional_int(conn.query_params["author_id"])
    published = conn.query_params["published"] == "true"
    result = Store.list_posts(page, per_page, author_id, published)
    result = %{result | items: Enum.map(result.items, &Post.to_json/1)}
    Router.send_json(conn, 200, result)
  end

  post "/" do
    conn = fetch_query_params(conn)
    author_id = String.to_integer(conn.query_params["author_id"])
    true = Store.get_user(author_id) != nil
    post = Store.create_post(author_id, conn.body_params)
    Router.send_json(conn, 201, Post.to_json(post))
  end

  get "/:id" do
    case Store.get_post(String.to_integer(id)) do
      nil -> Router.send_json(conn, 404, %{detail: "Post not found"})
      post -> Router.send_json(conn, 200, Post.to_json(post))
    end
  end

  patch "/:id" do
    case Store.update_post(String.to_integer(id), conn.body_params) do
      nil -> Router.send_json(conn, 404, %{detail: "Post not found"})
      post -> Router.send_json(conn, 200, Post.to_json(post))
    end
  end

  delete "/:id" do
    case Store.delete_post(String.to_integer(id)) do
      false -> Router.send_json(conn, 404, %{detail: "Post not found"})
      true -> send_resp(conn, 204, "")
    end
  end

  defp parse_int(nil, default), do: default
  defp parse_int(val, _default), do: String.to_integer(val)

  defp parse_optional_int(nil), do: nil
  defp parse_optional_int(val), do: String.to_integer(val)
end
