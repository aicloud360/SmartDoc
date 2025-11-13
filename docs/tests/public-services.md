# 公共服务（Auth/API）独立测试方案

## 前置条件
- Python 3.11+
- 已复制 `.env.example` 为 `.env`，并根据实际 DocumentServer/NAS 调整。

## 步骤
```bash
# 方式一：本地虚拟环境
cd services/api
python3 -m venv .venv && source .venv/bin/activate
pip install -r requirements.txt
export DATABASE_URL="postgresql+psycopg2://smartdoc:smartdoc@localhost:55432/smartdoc"
uvicorn app.main:app --reload --port 9100

# 方式二：Docker Compose
cd <project-root>
docker compose -f docker-compose.auth.yml up -d --build
```

## 验证
1. 健康检查
   ```bash
   curl http://localhost:9100/health
   ```
2. 登录获取 Token（占位逻辑）
   ```bash
   curl -X POST http://localhost:9100/auth/register \
     -H 'Content-Type: application/json' \
     -d '{"username":"demo","password":"demo","email":"demo@example.com"}'
   curl -X POST http://localhost:9100/auth/login \
     -H 'Content-Type: application/json' \
     -d '{"username":"demo","password":"demo"}'
   ```
3. DocumentServer Token
   ```bash
   curl -X POST http://localhost:9100/document/token \
     -H 'Content-Type: application/json' \
     -d '{"file_id":"demo.docx"}'
   ```

## 期望
- `/health` 返回 status=ok。
- `/auth/register` + `/auth/login` 返回 `access_token`（HS256 JWT）。
- `/document/token` 返回 `token` 与 `document_url`。

## TODO
- 添加 pytest 单测、NAS Docker Compose、DocumentServer 回调模拟。
