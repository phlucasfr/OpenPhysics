from fastapi import APIRouter

router = APIRouter()


@router.post("/")
def hello_physics():
    return "hello physics"
