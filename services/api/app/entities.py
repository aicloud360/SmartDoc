from datetime import datetime
from typing import Optional
import uuid

from sqlalchemy import Boolean, Column, DateTime, String
from sqlalchemy.dialects.postgresql import UUID

from .db import Base


class UserEntity(Base):
    __tablename__ = "users"

    id = Column(UUID(as_uuid=True), primary_key=True, default=uuid.uuid4)
    username = Column(String(64), unique=True, nullable=False, index=True)
    hashed_password = Column(String(255), nullable=False)
    email = Column(String(255), unique=True, nullable=True)
    role = Column(String(32), nullable=False, default="user")
    is_active = Column(Boolean, default=True)
    created_at = Column(DateTime, default=datetime.utcnow)
