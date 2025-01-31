/// Format of the frame following after
pub enum Format {
    I = 0b00,
    S = 0b01,
    U = 0b11,
}

/// Type Identification, 1 byte, 0 is not used
#[repr(u8)]
pub enum TypeId {
    // Not exhaustive due to reasons
    // TODO: Mapping usable names => standard names and maybe back?
    // TOOD: Mapping names => Data type decoders + byte counts
    SinglePointInfo = 1, //  M_SP_NA_1
    DoublePointInfo = 3,
    MeasuredValueNormalized = 9,
    MeasuredValueScaled = 11,
    MeasuredValueFloat = 13,
    IntegratedTotal = 15,
    SinglePointInfoWithTimeTag = 30,
    DoublePointInfoWithTimeTag = 31,
    MeasuredValueNormalizedWithTimeTag = 34,
    MeasuredValueScaledWithTimeTag = 35,
    MeasuredValueFloatWithTimeTag = 36,
    IntegratedTotalWithTimeTag = 37,
    SingleCommand = 45,
    DoubleCommand = 46,
    RegulatingSetpCommand = 47,
    SetpointCommand = 50,
    InterrogationCommand = 100,
    ReadCommand = 102,
    ResetProcessCommand = 105,
}

impl TypeId {
    // TODO: This should perhaps be "Display" or "Debug"?
    pub fn to_str(&self) -> &'static str {
        use TypeId::*;
        match self {
            SinglePointInfo => "M_SP_NA_1",
            DoublePointInfo => "M_DP_NA_1",
            MeasuredValueNormalized => "M_ME_NA_1",
            MeasuredValueScaled => "M_ME_NB_1",
            MeasuredValueFloat => "M_ME_NC_1",
            IntegratedTotal => "M_IT_NA_1",
            SinglePointInfoWithTimeTag => "M_SP_TB_1",
            DoublePointInfoWithTimeTag => "M_DP_TB_1",
            MeasuredValueNormalizedWithTimeTag => "M_ME_TD_1",
            MeasuredValueScaledWithTimeTag => "M_ME_TE_1",
            MeasuredValueFloatWithTimeTag => "M_ME_TF_1",
            IntegratedTotalWithTimeTag => "M_IT_TB_1",
            SingleCommand => "C_SC_NA_1",
            DoubleCommand => "C_DC_NA_1",
            RegulatingSetpCommand => "C_RC_NA_1",
            SetpointCommand => "C_SE_NC_1",
            InterrogationCommand => "C_IC_NA_1",
            ReadCommand => "C_RD_NA_1",
            ResetProcessCommand => "C_RP_NA_1",
        }
    }
}

/// Cause of transmission, 6 bits
pub enum Cot {
    // TODO: The names are incoherent. map them to something from wireshark or otherwise useful?
    PerCyc = 1,   // Periodic / Cyclic
    Back = 2,     // Background interrogation
    Spont = 3,    // Spontaneous
    Init = 4,     // Initialized
    Req = 5,      // Request or Interrogation
    Act = 6,      // Activation
    ActCon = 7,   // Activation Confirmation
    Deact = 8,    // Deactivation
    DeactCon = 9, // Deactivation Confirmation
    ActTerm = 10, // Termination Activation
    RetRem = 11,  // Feedback caused by distant commant
    RetLoc = 12,  // Feedback caused by local command
    File = 13,    // Data transmission
    Inrogen = 20, // Interrogated by general interrogation
    Inro1 = 21,   //interrogated by interrogation group 1
    Inro2 = 22,
    Inro3 = 23,
    Inro4 = 24,
    Inro5 = 25,
    Inro6 = 26,
    Inro7 = 27,
    Inro8 = 28,
    Inro9 = 29,
    Inro10 = 30,
    Inro11 = 31,
    Inro12 = 32,
    Inro13 = 33,
    Inro14 = 34,
    Inro15 = 35,
    Inro16 = 36,
    UnknownType = 44,                     // Type Identification unknown
    CauseUnknown = 45,                    // CoT Unknown
    AsduAddressUnknown = 46,              // ASDU Address Unknown
    InformationObjectAddressUnknown = 47, // IOA Unknown
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name_parse() {
        let type_id = TypeId::InterrogationCommand;
        let result = type_id.to_str();
        assert_eq!(result, "C_IC_NA_1");
    }

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
