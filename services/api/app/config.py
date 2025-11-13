from pydantic_settings import BaseSettings, SettingsConfigDict
from pydantic import AnyHttpUrl, Field


class Settings(BaseSettings):
    model_config = SettingsConfigDict(env_file=".env", env_file_encoding="utf-8")

    document_server_url: AnyHttpUrl = Field(
        default="http://10.18.65.129:8085/example/",
        description="OnlyOffice DocumentServer base url",
    )
    document_server_secret: str = Field(default="smartdoc-local-secret")
    jwt_issuer: str = Field(default="smartdoc")
    jwt_expires_seconds: int = Field(default=3600)
    database_url: str = Field(
        default="postgresql+psycopg2://smartdoc:smartdoc@localhost:55432/smartdoc",
        description="SQLAlchemy-compatible database URL",
    )


settings = Settings()
