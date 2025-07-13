from fastapi import APIRouter
from app.domain.api_models import ApiResponse
from app.services.mechanics_service import vector_velocity
from app.domain.mechanics_models import VectorVelocityRequest

router = APIRouter()


@router.post("/")
def hello_physics():
    return "hello physics"


@router.post("/vector-velocity", response_model=ApiResponse)
def calculate_vector_velocity(data: VectorVelocityRequest):
    return ApiResponse(
        data=vector_velocity(data),
        message="vector_velocity"
    )
