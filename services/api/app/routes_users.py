from fastapi import APIRouter, Depends
from sqlalchemy.orm import Session

from .db import get_session
from .repository import list_users
from .schemas import UserInfoResponse

router = APIRouter(prefix="/users", tags=["users"])


@router.get("/", response_model=list[UserInfoResponse])
def list_all_users(session: Session = Depends(get_session)) -> list[UserInfoResponse]:
    return [UserInfoResponse(username=user.username, role=user.role) for user in list_users(session)]
