defmodule RestApi.Models do
  @roles ~w(admin user moderator)

  def roles, do: @roles

  defmodule User do
    defstruct [:id, :name, :email, :role, :created_at, :updated_at]

    def to_json(%__MODULE__{} = u) do
      %{
        id: u.id,
        name: u.name,
        email: u.email,
        role: u.role,
        created_at: u.created_at,
        updated_at: u.updated_at
      }
    end
  end

  defmodule Post do
    defstruct [:id, :author_id, :title, :body, :published, :created_at, :updated_at]

    def to_json(%__MODULE__{} = p) do
      %{
        id: p.id,
        author_id: p.author_id,
        title: p.title,
        body: p.body,
        published: p.published,
        created_at: p.created_at,
        updated_at: p.updated_at
      }
    end
  end

  def paginate(items, page, per_page) do
    total = length(items)
    start = (page - 1) * per_page
    page_items = items |> Enum.drop(start) |> Enum.take(per_page)
    pages = div(total + per_page - 1, per_page)
    %{items: page_items, total: total, page: page, per_page: per_page, pages: pages}
  end
end
