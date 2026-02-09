import logging
import time

from fastapi import Request, Response

logger = logging.getLogger("rest_api")


async def logging_middleware(request: Request, call_next) -> Response:
    start = time.monotonic()
    response = await call_next(request)
    elapsed_ms = (time.monotonic() - start) * 1000
    path = request.url.path
    logger.info(f"{request.method} {path} -> {response.status_code} ({elapsed_ms:.1f}ms)")
    return response
