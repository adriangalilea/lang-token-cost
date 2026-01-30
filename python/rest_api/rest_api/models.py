from datetime import datetime
from enum import StrEnum

from pydantic import BaseModel, EmailStr, Field


class Role(StrEnum):
    ADMIN = "admin"
    USER = "user"
    MODERATOR = "moderator"


class UserCreate(BaseModel):
    name: str = Field(min_length=1, max_length=100)
    email: str = Field(min_length=1)
    role: Role = Role.USER


class UserUpdate(BaseModel):
    name: str | None = Field(default=None, min_length=1, max_length=100)
    email: str | None = Field(default=None, min_length=1)
    role: Role | None = None


class User(BaseModel):
    id: int
    name: str
    email: str
    role: Role
    created_at: datetime
    updated_at: datetime


class PostCreate(BaseModel):
    title: str = Field(min_length=1, max_length=200)
    body: str = Field(min_length=1)
    published: bool = False


class PostUpdate(BaseModel):
    title: str | None = Field(default=None, min_length=1, max_length=200)
    body: str | None = Field(default=None, min_length=1)
    published: bool | None = None


class Post(BaseModel):
    id: int
    author_id: int
    title: str
    body: str
    published: bool
    created_at: datetime
    updated_at: datetime


class PaginatedResponse[T](BaseModel):
    items: list[T]
    total: int
    page: int
    per_page: int
    pages: int
