from fastapi import APIRouter, HTTPException, Query

from .models import PaginatedResponse, User, UserCreate, UserUpdate
from .store import store

router = APIRouter(prefix="/users", tags=["users"])


@router.get("", response_model=PaginatedResponse[User])
def list_users(
    page: int = Query(1, ge=1),
    per_page: int = Query(20, ge=1, le=100),
    search: str | None = None,
) -> PaginatedResponse[User]:
    return store.list_users(page, per_page, search)


@router.post("", response_model=User, status_code=201)
def create_user(body: UserCreate) -> User:
    return store.create_user(body)


@router.get("/{user_id}", response_model=User)
def get_user(user_id: int) -> User:
    user = store.get_user(user_id)
    if user is None:
        raise HTTPException(404, "User not found")
    return user


@router.patch("/{user_id}", response_model=User)
def update_user(user_id: int, body: UserUpdate) -> User:
    user = store.update_user(user_id, body)
    if user is None:
        raise HTTPException(404, "User not found")
    return user


@router.delete("/{user_id}", status_code=204)
def delete_user(user_id: int) -> None:
    if not store.delete_user(user_id):
        raise HTTPException(404, "User not found")
