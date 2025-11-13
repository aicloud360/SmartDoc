# SmartDoc Public Services (FastAPI)

轻量鉴权 / DocumentServer 网关，运行在 NAS 或任意 Docker 环境。

## 目录结构
```
services/api/
├── app/
│   ├── __init__.py
│   ├── main.py
│   ├── config.py
│   ├── schemas.py
│   └── services.py
├── requirements.txt
├── README.md
└── .env.example
```

## 快速开始
### 本地虚拟环境
```bash
cd services/api
python3 -m venv .venv && source .venv/bin/activate
pip install -r requirements.txt
export DATABASE_URL="postgresql+psycopg2://smartdoc:smartdoc@localhost:55432/smartdoc"
uvicorn app.main:app --reload --port 9100
```

### Docker Compose（推荐，用于 Web 服务器 / NAS）
```bash
cd /Users/.../SmartDoc
docker compose -f docker-compose.auth.yml up -d --build
```
服务暴露：
- API: `http://localhost:9100`
- Postgres: `localhost:55432`（避免与本机默认 5432 冲突）

### API 验证
- 健康检查：`GET http://localhost:9100/health`
- 注册：`POST /auth/register {"username":"demo","password":"demo","email":"demo@example.com"}`
- 登录：`POST /auth/login {"username":"demo","password":"demo"}`
- 列表：`GET /users/`
- DocumentServer token：`POST /document/token {"file_id":"123"}`

## 单测
```bash
curl -X POST http://localhost:9100/auth/register \
     -H "Content-Type: application/json" \
     -d '{"username":"demo","password":"demo","email":"demo@example.com"}'
     
curl -X POST http://localhost:9100/auth/login \
     -H "Content-Type: application/json" \
     -d '{"username":"demo","password":"demo"}'

curl -X GET http://localhost:9100/users/ \
     -H "Authorization: Bearer <YOUR_JWT_TOKEN_HERE>"

curl -X POST http://localhost:9100/document/token \
     -H "Content-Type: application/json" \
     -H "Authorization: Bearer <YOUR_JWT_TOKEN_HERE>" \
     -d '{"file_id":"123"}'
```

## 环境变量
参见 `.env.example`：
- `DOCUMENT_SERVER_URL`
- `DOCUMENT_SERVER_SECRET`
- `JWT_ISSUER`
- `JWT_EXPIRES_SECONDS`

## 后续计划
- 接入真实用户目录（NAS/若依/SSO）。
- 校验 DocumentServer 回调、签名权限。
- 与桌面/Web/移动端联调。
