from datetime import datetime

from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware

from .config import settings
from .routes_auth import router as auth_router
from .routes_users import router as users_router
from .schemas import DocumentTokenRequest, DocumentTokenResponse, HealthResponse
from .services import create_document_token
from .db import Base, engine

app = FastAPI(title="SmartDoc Public Services", version="0.1.0")

app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)


@app.on_event("startup")
def startup() -> None:
    Base.metadata.create_all(bind=engine)


@app.get("/health", response_model=HealthResponse)
def health_check() -> HealthResponse:
    return HealthResponse(status="ok", timestamp=datetime.utcnow())


app.include_router(auth_router)
app.include_router(users_router)


@app.post("/document/token", response_model=DocumentTokenResponse)
def document_token(payload: DocumentTokenRequest) -> DocumentTokenResponse:
    token, expires_at = create_document_token(payload.file_id, payload.permissions)
    url = f"{settings.document_server_url}?fileId={payload.file_id}"
    return DocumentTokenResponse(token=token, expires_at=expires_at, document_url=url)
