from datetime import datetime, timedelta
from typing import Optional

from pydantic import BaseModel, Field


class HealthResponse(BaseModel):
    status: str = "ok"
    timestamp: datetime


class LoginRequest(BaseModel):
    username: str
    password: str


class RegisterRequest(LoginRequest):
    email: str | None = None


class TokenResponse(BaseModel):
    access_token: str
    token_type: str = "bearer"
    expires_in: int


class DocumentTokenRequest(BaseModel):
    file_id: str
    permissions: Optional[dict] = Field(default_factory=dict)


class DocumentTokenResponse(BaseModel):
    token: str
    expires_at: datetime
    document_url: str


class UserInfoResponse(BaseModel):
    username: str
    role: str
