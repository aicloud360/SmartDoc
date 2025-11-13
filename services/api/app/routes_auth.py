from fastapi import APIRouter, Depends, HTTPException, status
from passlib.context import CryptContext
from sqlalchemy.orm import Session

from .schemas import LoginRequest, RegisterRequest, TokenResponse
from .services import create_access_token
from .db import get_session
from .repository import create_user, get_user_by_username

router = APIRouter(prefix="/auth", tags=["auth"])
pwd_context = CryptContext(schemes=["bcrypt"], deprecated="auto")


def verify_password(plain_password: str, hashed_password: str) -> bool:
    return pwd_context.verify(plain_password, hashed_password)


def hash_password(password: str) -> str:
    return pwd_context.hash(password)


@router.post("/register", response_model=TokenResponse)
def register(payload: RegisterRequest, session: Session = Depends(get_session)) -> TokenResponse:
    if get_user_by_username(session, payload.username):
        raise HTTPException(status_code=status.HTTP_409_CONFLICT, detail="用户已存在")
    user = create_user(
        session,
        username=payload.username,
        hashed_password=hash_password(payload.password),
        email=payload.email,
    )
    token, expires_in = create_access_token(user.username)
    return TokenResponse(access_token=token, expires_in=expires_in)


@router.post("/login", response_model=TokenResponse)
def login(payload: LoginRequest, session: Session = Depends(get_session)) -> TokenResponse:
    user = get_user_by_username(session, payload.username)
    if user is None or not verify_password(payload.password, user.hashed_password):
        raise HTTPException(status_code=status.HTTP_401_UNAUTHORIZED, detail="用户名或密码错误")
    token, expires_in = create_access_token(user.username)
    return TokenResponse(access_token=token, expires_in=expires_in)
