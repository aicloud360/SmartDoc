from pydantic import BaseModel
from typing import Literal
from datetime import datetime

class User(BaseModel):
    id: str
    username: str
    hashed_password: str
    email: str | None = None
    role: Literal["admin", "user"] = "user"
    created_at: datetime

USERS_DB: dict[str, User] = {}
