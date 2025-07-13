from app.domain.mechanics_models import (
    VectorVelocityRequest,
    VectorVelocityResponse,
)


def vector_velocity(data: VectorVelocityRequest) -> VectorVelocityResponse:
    return VectorVelocityResponse(
        velocity=data.displacement_change / data.time_change,
        direction=data.direction,
        velocity_unit=f"{data.displacement_unit}/{data.time_unit}"
    )
