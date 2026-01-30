from fastapi import APIRouter, HTTPException, Query

from .models import PaginatedResponse, Post, PostCreate, PostUpdate
from .store import store

router = APIRouter(prefix="/posts", tags=["posts"])


@router.get("", response_model=PaginatedResponse[Post])
def list_posts(
    page: int = Query(1, ge=1),
    per_page: int = Query(20, ge=1, le=100),
    author_id: int | None = None,
    published: bool = False,
) -> PaginatedResponse[Post]:
    return store.list_posts(page, per_page, author_id, published)


@router.post("", response_model=Post, status_code=201)
def create_post(body: PostCreate, author_id: int = Query(...)) -> Post:
    assert store.get_user(author_id) is not None, f"Author {author_id} not found"
    return store.create_post(author_id, body)


@router.get("/{post_id}", response_model=Post)
def get_post(post_id: int) -> Post:
    post = store.get_post(post_id)
    if post is None:
        raise HTTPException(404, "Post not found")
    return post


@router.patch("/{post_id}", response_model=Post)
def update_post(post_id: int, body: PostUpdate) -> Post:
    post = store.update_post(post_id, body)
    if post is None:
        raise HTTPException(404, "Post not found")
    return post


@router.delete("/{post_id}", status_code=204)
def delete_post(post_id: int) -> None:
    if not store.delete_post(post_id):
        raise HTTPException(404, "Post not found")
