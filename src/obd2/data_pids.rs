use crate::enum_wrapper;
use enum2repr::EnumRepr;

/// OBD2 data PIDs used for Service 01 and 02
#[repr(u8)]
#[derive(EnumRepr, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum DataPid {
    PidSupport0120 = 0x00,
    StatusSinceDTCCleared = 0x01,
    FreezeDTC = 0x02,
    FuelSystemStatus = 0x03,
    CalculatedEngineLoad = 0x04,
    EngineCoolantTemp = 0x05,
    ShortTermFuelTrimBank1 = 0x06,
    LongTermFuelTrimBank1 = 0x07,
    ShortTermFuelTrimBank2 = 0x08,
    LongTermFuelTrimBank2 = 0x09,
    FuelPressureGauge = 0x0A,
    IntakeManifoldAbsPressure = 0x0B,
    EngineSpeed = 0x0C,
    VehicleSpeed = 0x0D,
    TimingAdvance = 0x0E,
    IntakeAirTemperature = 0x0F,
    MassAirFlow = 0x10,
    ThrottlePosition = 0x11,
    CommandedSecondaryAirStatus = 0x12,
    O2SensorsPresent2Banks = 0x13,
    OxygenSensor1 = 0x14,
    OxygenSensor2 = 0x15,
    OxygenSensor3 = 0x16,
    OxygenSensor4 = 0x17,
    OxygenSensor5 = 0x18,
    OxygenSensor6 = 0x19,
    OxygenSensor7 = 0x1A,
    OxygenSensor8 = 0x1B,
    ObdStandard = 0x1C,
    O2SensorsPresent4Banks = 0x1D,
    AuxInputStatus = 0x1E,
    RuntimeSinceStart = 0x1F,
    PidSupport2140 = 0x20,
    MILRuntime = 0x21,
    FuelRailPressure = 0x22,
    FuelRailGaugePressure = 0x23,
    OxygenSensor1LambdaVoltage = 0x24,
    OxygenSensor2LambdaVoltage = 0x25,
    OxygenSensor3LambdaVoltage = 0x26,
    OxygenSensor4LambdaVoltage = 0x27,
    OxygenSensor5LambdaVoltage = 0x28,
    OxygenSensor6LambdaVoltage = 0x29,
    OxygenSensor7LambdaVoltage = 0x2A,
    OxygenSensor8LambdaVoltage = 0x2B,
    CommandedEGR = 0x2C,
    EGRError = 0x2D,
    CommandedEvapPurge = 0x2E,
    FuelTankLevelInput = 0x2F,
    WarmupsSinceCodesCleared = 0x30,
    DistanceTraveledSinceCodesCleared = 0x31,
    EvapSystemVaporPressure = 0x32,
    AbsBarometricPressure = 0x33,
    OxygenSensor1LambdaCurrent = 0x34,
    OxygenSensor2LambdaCurrent = 0x35,
    OxygenSensor3LambdaCurrent = 0x36,
    OxygenSensor4LambdaCurrent = 0x37,
    OxygenSensor5LambdaCurrent = 0x38,
    OxygenSensor6LambdaCurrent = 0x39,
    OxygenSensor7LambdaCurrent = 0x3A,
    OxygenSensor8LambdaCurrent = 0x3B,
    CatTempBank1Sensor1 = 0x3C,
    CatTempBank2Sensor1 = 0x3D,
    CatTempBank1Sensor2 = 0x3E,
    CatTempBank2Sensor2 = 0x3F,
    // FIXME: this was set to 0x20 in the original, but parsed from 0x40, so assume the correct value is 0x20
    PidSupport4160 = 0x40,
    MonitorStatusDriveCycle = 0x41,
    ControlModuleVoltage = 0x42,
    AbsLoadValue = 0x43,
    CommandedLambda = 0x44,
    RelativeThrottlePosition = 0x45,
    AmbientAirTemp = 0x46,
    AbsoluteThrottlePositionB = 0x47,
    AbsoluteThrottlePositionC = 0x48,
    AbsoluteThrottlePositionD = 0x49,
    AbsoluteThrottlePositionE = 0x4A,
    AbsoluteThrottlePositionF = 0x4B,
    CommandedThrottleActuator = 0x4C,
    TimeRunSinceMILOn = 0x4D,
    TimeSinceCodesCleared = 0x4E,
    MaximumLambdaVoltageCurrentPressure = 0x4F,
    MaximumAirFlowRate = 0x50,
    FuelType = 0x51,
    EthanolFuelPercentage = 0x52,
    AbsoluteEvapSystemVaporPressure = 0x53,
    EvapSystemVaporPressure2 = 0x54,
    ShortTermSecondaryOxygenSensorTrimBank1and3 = 0x55,
    LongTermSecondaryOxygenSensorTrimBank1and3 = 0x56,
    ShortTermSecondaryOxygenSensorTrimBank2and4 = 0x57,
    LongTermSecondaryOxygenSensorTrimBank2and4 = 0x58,
    FuelRailAbsPressure = 0x59,
    RelativePedalPosition = 0x5A,
    HybridBatteryPackLife = 0x5B,
    EngineOilTemp = 0x5C,
    FuelInjectionTiming = 0x5D,
    EngineFuelRate = 0x5E,
    EmissionsStandard = 0x5F,
    PidSupport6180 = 0x60,
    DriverDemandTorquePercent = 0x61,
    EngineTorquePercent = 0x62,
    EngineTorqueData = 0x63,
    AuxInputOutputSupport = 0x64,
    MassAirFlowSensor2 = 0x65,
    EngineCoolantTemp2 = 0x66,
    IntakeAirTemp2 = 0x67,
}

enum_wrapper!(obd2, DataPid, DataPidByte);
