from sqlalchemy.orm import Session

from .entities import UserEntity


def get_user_by_username(session: Session, username: str) -> UserEntity | None:
    return session.query(UserEntity).filter(UserEntity.username == username).first()


def create_user(session: Session, username: str, hashed_password: str, email: str | None = None, role: str = "user") -> UserEntity:
    user = UserEntity(username=username, hashed_password=hashed_password, email=email, role=role)
    session.add(user)
    session.commit()
    session.refresh(user)
    return user


def list_users(session: Session) -> list[UserEntity]:
    return session.query(UserEntity).order_by(UserEntity.created_at.desc()).all()
