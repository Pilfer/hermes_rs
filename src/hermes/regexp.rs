// https://github.com/facebook/hermes/blob/main/include/hermes/Regex/RegexBytecode.h
// https://github.com/facebook/hermes/blob/main/include/hermes/Regex/RegexOpcodes.def


// RegexBytecodeHeader
pub struct RegexBytecodeHeader {
  // Number of capture groups.
  u16 markedCount,

  // Number of loops.
  u16 loopCount,

  // Syntax flags used to construct the regex.
  u8 syntaxFlags,

  /// Constraints on what strings can match this regex.
  MatchConstraintSet constraints,
}