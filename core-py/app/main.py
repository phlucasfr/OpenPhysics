from fastapi import FastAPI
from app.api.mechanics_router import router as mechanics_router

app = FastAPI(title="OpenPhysics a Physics API")
app.include_router(mechanics_router, prefix="/mechanics")
