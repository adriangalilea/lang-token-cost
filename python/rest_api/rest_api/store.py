from datetime import datetime

from .models import (
    PaginatedResponse,
    Post,
    PostCreate,
    PostUpdate,
    User,
    UserCreate,
    UserUpdate,
)


class Store:
    def __init__(self) -> None:
        self.users: dict[int, User] = {}
        self.posts: dict[int, Post] = {}
        self._user_id = 0
        self._post_id = 0

    def _next_user_id(self) -> int:
        self._user_id += 1
        return self._user_id

    def _next_post_id(self) -> int:
        self._post_id += 1
        return self._post_id

    def _post_count(self, user_id: int) -> int:
        return sum(1 for p in self.posts.values() if p.author_id == user_id)

    def _with_post_count(self, user: User) -> User:
        return user.model_copy(update={"post_count": self._post_count(user.id)})

    def create_user(self, data: UserCreate) -> User:
        now = datetime.now()
        user = User(id=self._next_user_id(), **data.model_dump(), created_at=now, updated_at=now)
        self.users[user.id] = user
        return self._with_post_count(user)

    def get_user(self, user_id: int) -> User | None:
        user = self.users.get(user_id)
        return self._with_post_count(user) if user else None

    def list_users(self, page: int, per_page: int, search: str | None) -> PaginatedResponse[User]:
        items = list(self.users.values())
        if search:
            query = search.lower()
            items = [u for u in items if query in u.name.lower() or query in u.email.lower()]
        total = len(items)
        start = (page - 1) * per_page
        items = [self._with_post_count(u) for u in items[start : start + per_page]]
        return PaginatedResponse(
            items=items,
            total=total,
            page=page,
            per_page=per_page,
            pages=(total + per_page - 1) // per_page,
        )

    def update_user(self, user_id: int, data: UserUpdate) -> User | None:
        user = self.users.get(user_id)
        if user is None:
            return None
        updates = data.model_dump(exclude_unset=True)
        updated = user.model_copy(update={**updates, "updated_at": datetime.now()})
        self.users[user_id] = updated
        return self._with_post_count(updated)

    def delete_user(self, user_id: int) -> bool:
        if self.users.pop(user_id, None) is None:
            return False
        self.posts = {k: v for k, v in self.posts.items() if v.author_id != user_id}
        return True

    def create_post(self, author_id: int, data: PostCreate) -> Post:
        now = datetime.now()
        post = Post(
            id=self._next_post_id(),
            author_id=author_id,
            **data.model_dump(),
            created_at=now,
            updated_at=now,
        )
        self.posts[post.id] = post
        return post

    def get_post(self, post_id: int) -> Post | None:
        return self.posts.get(post_id)

    def list_posts(
        self,
        page: int,
        per_page: int,
        author_id: int | None,
        published_only: bool,
    ) -> PaginatedResponse[Post]:
        items = list(self.posts.values())
        if author_id is not None:
            items = [p for p in items if p.author_id == author_id]
        if published_only:
            items = [p for p in items if p.published]
        total = len(items)
        start = (page - 1) * per_page
        items = items[start : start + per_page]
        return PaginatedResponse(
            items=items,
            total=total,
            page=page,
            per_page=per_page,
            pages=(total + per_page - 1) // per_page,
        )

    def update_post(self, post_id: int, data: PostUpdate) -> Post | None:
        post = self.posts.get(post_id)
        if post is None:
            return None
        updates = data.model_dump(exclude_unset=True)
        updated = post.model_copy(update={**updates, "updated_at": datetime.now()})
        self.posts[post_id] = updated
        return updated

    def delete_post(self, post_id: int) -> bool:
        return self.posts.pop(post_id, None) is not None


store = Store()
