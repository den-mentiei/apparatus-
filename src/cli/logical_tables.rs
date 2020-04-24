// Taken from ECMA II.22

const METADATA_MODULE:                 usize = 0x00;
const METADATA_TYPEREF:                usize = 0x01;
const METADATA_TYPEDEF:                usize = 0x02;
const METADATA_FIELD:                  usize = 0x04;
const METADATA_METHODDEF:              usize = 0x06;
const METADATA_PARAM:                  usize = 0x08;
const METADATA_INTERFACEIMPL:          usize = 0x09;
const METADATA_MEMBERREF:              usize = 0x0A;
const METADATA_CONSTANT:               usize = 0x0B;
const METADATA_CUSTOMATTRIBUTE:        usize = 0x0C;
const METADATA_FIELDMARSHAL:           usize = 0x0D;
const METADATA_DECLSECURITY:           usize = 0x0E;
const METADATA_CLASSLAYOUT:            usize = 0x0F;
const METADATA_FIELDLAYOUT:            usize = 0x10;
const METADATA_STANDALONESIG:          usize = 0x11;
const METADATA_EVENTMAP:               usize = 0x12;
const METADATA_EVENT:                  usize = 0x14;
const METADATA_PROPERTYMAP:            usize = 0x15;
const METADATA_PROPERTY:               usize = 0x17;
const METADATA_METHODSEMANTICS:        usize = 0x18;
const METADATA_METHODIMPL:             usize = 0x19;
const METADATA_MODULEREF:              usize = 0x1A;
const METADATA_TYPESPEC:               usize = 0x1B;
const METADATA_IMPLMAP:                usize = 0x1C;
const METADATA_FIELDRVA:               usize = 0x1D;
const METADATA_ASSEMBLY:               usize = 0x20;
const METADATA_ASSEMBLYPROCESSOR:      usize = 0x21;
const METADATA_ASSEMBLYOS:             usize = 0x22;
const METADATA_ASSEMBLYREF:            usize = 0x23;
const METADATA_ASSEMBLYREFPROCESSOR:   usize = 0x24;
const METADATA_ASSEMBLYREFOS:          usize = 0x25;
const METADATA_FILE:                   usize = 0x26;
const METADATA_EXPORTEDTYPE:           usize = 0x27;
const METADATA_MANIFESTRESOURCE:       usize = 0x28;
const METADATA_NESTEDCLASS:            usize = 0x29;
const METADATA_GENERICPARAM:           usize = 0x2A;
const METADATA_METHODSPEC:             usize = 0x2B;
const METADATA_GENERICPARAMCONSTRAINT: usize = 0x2C;
