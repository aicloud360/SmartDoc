from datetime import datetime, timedelta

import jwt

from .config import settings


def create_access_token(subject: str) -> tuple[str, int]:
    expires_delta = timedelta(seconds=settings.jwt_expires_seconds)
    expire_ts = datetime.utcnow() + expires_delta
    payload = {
        "sub": subject,
        "iss": settings.jwt_issuer,
        "exp": expire_ts,
        "iat": datetime.utcnow(),
    }
    token = jwt.encode(payload, settings.document_server_secret, algorithm="HS256")
    return token, settings.jwt_expires_seconds


def create_document_token(file_id: str, permissions: dict | None = None) -> tuple[str, datetime]:
    expires_delta = timedelta(seconds=settings.jwt_expires_seconds)
    expire_ts = datetime.utcnow() + expires_delta
    payload = {
        "file_id": file_id,
        "permissions": permissions or {},
        "iss": settings.jwt_issuer,
        "exp": expire_ts,
    }
    token = jwt.encode(payload, settings.document_server_secret, algorithm="HS256")
    return token, expire_ts
