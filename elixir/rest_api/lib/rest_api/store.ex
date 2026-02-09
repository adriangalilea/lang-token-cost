defmodule RestApi.Store do
  use Agent

  alias RestApi.Models.{User, Post}

  def start_link(_),
    do:
      Agent.start_link(fn -> %{users: %{}, posts: %{}, user_id: 0, post_id: 0} end,
        name: __MODULE__
      )

  def create_user(data) do
    Agent.get_and_update(__MODULE__, fn state ->
      id = state.user_id + 1
      now = DateTime.utc_now() |> DateTime.to_iso8601()

      user = %User{
        id: id,
        name: data["name"],
        email: data["email"],
        role: data["role"] || "user",
        created_at: now,
        updated_at: now
      }

      state = %{state | users: Map.put(state.users, id, user), user_id: id}
      {user, state}
    end)
  end

  def get_user(id), do: Agent.get(__MODULE__, &Map.get(&1.users, id))

  def list_users(page, per_page, search) do
    Agent.get(__MODULE__, fn state ->
      items = Map.values(state.users)

      items =
        if search do
          q = String.downcase(search)

          Enum.filter(
            items,
            &(String.contains?(String.downcase(&1.name), q) or
                String.contains?(String.downcase(&1.email), q))
          )
        else
          items
        end

      RestApi.Models.paginate(items, page, per_page)
    end)
  end

  def update_user(id, data) do
    Agent.get_and_update(__MODULE__, fn state ->
      case Map.get(state.users, id) do
        nil ->
          {nil, state}

        user ->
          now = DateTime.utc_now() |> DateTime.to_iso8601()
          updated = %{user | updated_at: now}
          updated = if data["name"], do: %{updated | name: data["name"]}, else: updated
          updated = if data["email"], do: %{updated | email: data["email"]}, else: updated
          updated = if data["role"], do: %{updated | role: data["role"]}, else: updated
          {updated, %{state | users: Map.put(state.users, id, updated)}}
      end
    end)
  end

  def delete_user(id) do
    Agent.get_and_update(__MODULE__, fn state ->
      if Map.has_key?(state.users, id) do
        posts = state.posts |> Enum.reject(fn {_, p} -> p.author_id == id end) |> Map.new()
        {true, %{state | users: Map.delete(state.users, id), posts: posts}}
      else
        {false, state}
      end
    end)
  end

  def count_user_posts(user_id) do
    Agent.get(__MODULE__, fn state ->
      Enum.count(state.posts, fn {_, p} -> p.author_id == user_id end)
    end)
  end

  def create_post(author_id, data) do
    Agent.get_and_update(__MODULE__, fn state ->
      id = state.post_id + 1
      now = DateTime.utc_now() |> DateTime.to_iso8601()

      post = %Post{
        id: id,
        author_id: author_id,
        title: data["title"],
        body: data["body"],
        published: data["published"] || false,
        created_at: now,
        updated_at: now
      }

      state = %{state | posts: Map.put(state.posts, id, post), post_id: id}
      {post, state}
    end)
  end

  def get_post(id), do: Agent.get(__MODULE__, &Map.get(&1.posts, id))

  def list_posts(page, per_page, author_id, published_only) do
    Agent.get(__MODULE__, fn state ->
      items = Map.values(state.posts)
      items = if author_id, do: Enum.filter(items, &(&1.author_id == author_id)), else: items
      items = if published_only, do: Enum.filter(items, & &1.published), else: items
      RestApi.Models.paginate(items, page, per_page)
    end)
  end

  def update_post(id, data) do
    Agent.get_and_update(__MODULE__, fn state ->
      case Map.get(state.posts, id) do
        nil ->
          {nil, state}

        post ->
          now = DateTime.utc_now() |> DateTime.to_iso8601()
          updated = %{post | updated_at: now}
          updated = if data["title"], do: %{updated | title: data["title"]}, else: updated
          updated = if data["body"], do: %{updated | body: data["body"]}, else: updated

          updated =
            if Map.has_key?(data, "published"),
              do: %{updated | published: data["published"]},
              else: updated

          {updated, %{state | posts: Map.put(state.posts, id, updated)}}
      end
    end)
  end

  def delete_post(id) do
    Agent.get_and_update(__MODULE__, fn state ->
      if Map.has_key?(state.posts, id) do
        {true, %{state | posts: Map.delete(state.posts, id)}}
      else
        {false, state}
      end
    end)
  end
end
