use crate::hermes;

// Define all of the instructions that are used in the Hermes v95 bytecode
build_instructions!(
  (0, Unreachable, ),
  (1, NewObjectWithBuffer, r1: Reg8, p1: UInt16, p2: UInt16, p3: UInt16, p4: UInt16),
  (2, NewObjectWithBufferLong, r1: Reg8, p1: UInt16, p2: UInt16, p3: UInt32, p4: UInt32),
  (3, NewObject, r1: Reg8),
  (4, NewObjectWithParent, r1: Reg8, r2: Reg8),
  (5, NewArrayWithBuffer, r1: Reg8, p1: UInt16, p2: UInt16, p3: UInt16),
  (6, NewArrayWithBufferLong, r1: Reg8, p1: UInt16, p2: UInt16, p3: UInt32),
  (7, NewArray, r1: Reg8, p1: UInt16),
  (8, Mov, r1: Reg8, r2: Reg8),
  (9, MovLong, r1: Reg32, r2: Reg32),
  (10, Negate, r1: Reg8, r2: Reg8),
  (11, Not, r1: Reg8, r2: Reg8),
  (12, BitNot, r1: Reg8, r2: Reg8),
  (13, TypeOf, r1: Reg8, r2: Reg8),
  (14, Eq, r1: Reg8, r2: Reg8, r3: Reg8),
  (15, StrictEq, r1: Reg8, r2: Reg8, r3: Reg8),
  (16, Neq, r1: Reg8, r2: Reg8, r3: Reg8),
  (17, StrictNeq, r1: Reg8, r2: Reg8, r3: Reg8),
  (18, Less, r1: Reg8, r2: Reg8, r3: Reg8),
  (19, LessEq, r1: Reg8, r2: Reg8, r3: Reg8),
  (20, Greater, r1: Reg8, r2: Reg8, r3: Reg8),
  (21, GreaterEq, r1: Reg8, r2: Reg8, r3: Reg8),
  (22, Add, r1: Reg8, r2: Reg8, r3: Reg8),
  (23, AddN, r1: Reg8, r2: Reg8, r3: Reg8),
  (24, Mul, r1: Reg8, r2: Reg8, r3: Reg8),
  (25, MulN, r1: Reg8, r2: Reg8, r3: Reg8),
  (26, Div, r1: Reg8, r2: Reg8, r3: Reg8),
  (27, DivN, r1: Reg8, r2: Reg8, r3: Reg8),
  (28, Mod, r1: Reg8, r2: Reg8, r3: Reg8),
  (29, Sub, r1: Reg8, r2: Reg8, r3: Reg8),
  (30, SubN, r1: Reg8, r2: Reg8, r3: Reg8),
  (31, LShift, r1: Reg8, r2: Reg8, r3: Reg8),
  (32, RShift, r1: Reg8, r2: Reg8, r3: Reg8),
  (33, URshift, r1: Reg8, r2: Reg8, r3: Reg8),
  (34, BitAnd, r1: Reg8, r2: Reg8, r3: Reg8),
  (35, BitXor, r1: Reg8, r2: Reg8, r3: Reg8),
  (36, BitOr, r1: Reg8, r2: Reg8, r3: Reg8),
  (37, Inc, r1: Reg8, r2: Reg8),
  (38, Dec, r1: Reg8, r2: Reg8),
  (39, InstanceOf, r1: Reg8, r2: Reg8, r3: Reg8),
  (40, IsIn, r1: Reg8, r2: Reg8, r3: Reg8),
  (41, GetEnvironment, r1: Reg8, p1: UInt8),
  (42, StoreToEnvironment, r1: Reg8, p1: UInt8, r2: Reg8),
  (43, StoreToEnvironmentL, r1: Reg8, p1: UInt16, r2: Reg8),
  (44, StoreNPToEnvironment, r1: Reg8, p1: UInt8, r2: Reg8),
  (45, StoreNPToEnvironmentL, r1: Reg8, p1: UInt16, r2: Reg8),
  (46, LoadFromEnvironment, r1: Reg8, r2: Reg8, p1: UInt8),
  (47, LoadFromEnvironmentL, r1: Reg8, r2: Reg8, p1: UInt16),
  (48, GetGlobalObject, r1: Reg8),
  (49, GetNewTarget, r1: Reg8),
  (50, CreateEnvironment, r1: Reg8),
  (51, CreateInnerEnvironment, r1: Reg8, r2: Reg8, p1: UInt32),
  (52, DeclareGlobalVar, p1: StringIDUInt32),
  (53, ThrowIfHasRestrictedGlobalProperty, p1: StringIDUInt32),
  (54, GetByIdShort, r1: Reg8, r2: Reg8, p1: UInt8, p2: StringIDUInt8),
  (55, GetById, r1: Reg8, r2: Reg8, p1: UInt8, p2: StringIDUInt16),
  (56, GetByIdLong, r1: Reg8, r2: Reg8, p1: UInt8, p2: StringIDUInt32),
  (57, TryGetById, r1: Reg8, r2: Reg8, p1: UInt8, p2: StringIDUInt16),
  (58, TryGetByIdLong, r1: Reg8, r2: Reg8, p1: UInt8, p2: StringIDUInt32),
  (59, PutById, r1: Reg8, r2: Reg8, p1: UInt8, p2: StringIDUInt16),
  (60, PutByIdLong, r1: Reg8, r2: Reg8, p1: UInt8, p2: StringIDUInt32),
  (61, TryPutById, r1: Reg8, r2: Reg8, p1: UInt8, p2: StringIDUInt16),
  (62, TryPutByIdLong, r1: Reg8, r2: Reg8, p1: UInt8, p2: StringIDUInt32),
  (63, PutNewOwnByIdShort, r1: Reg8, r2: Reg8, p1: StringIDUInt8),
  (64, PutNewOwnById, r1: Reg8, r2: Reg8, p1: StringIDUInt16),
  (65, PutNewOwnByIdLong, r1: Reg8, r2: Reg8, p1: StringIDUInt32),
  (66, PutNewOwnNEById, r1: Reg8, r2: Reg8, p1: StringIDUInt16),
  (67, PutNewOwnNEByIdLong, r1: Reg8, r2: Reg8, p1: StringIDUInt32),
  (68, PutOwnByIndex, r1: Reg8, r2: Reg8, p1: UInt8),
  (69, PutOwnByIndexL, r1: Reg8, r2: Reg8, p1: UInt32),
  (70, PutOwnByVal, r1: Reg8, r2: Reg8, r3: Reg8, p1: UInt8),
  (71, DelById, r1: Reg8, r2: Reg8, p1: StringIDUInt16),
  (72, DelByIdLong, r1: Reg8, r2: Reg8, p1: StringIDUInt32),
  (73, GetByVal, r1: Reg8, r2: Reg8, r3: Reg8),
  (74, PutByVal, r1: Reg8, r2: Reg8, r3: Reg8),
  (75, DelByVal, r1: Reg8, r2: Reg8, r3: Reg8),
  (76, PutOwnGetterSetterByVal, r1: Reg8, r2: Reg8, r3: Reg8, r4: Reg8, p1: UInt8),
  (77, GetPNameList, r1: Reg8, r2: Reg8, r3: Reg8, r4: Reg8),
  (78, GetNextPName, r1: Reg8, r2: Reg8, r3: Reg8, r4: Reg8, r5: Reg8),
  (79, Call, r1: Reg8, r2: Reg8, p1: UInt8),
  (80, Construct, r1: Reg8, r2: Reg8, p1: UInt8),
  (81, Call1, r1: Reg8, r2: Reg8, r3: Reg8),
  (82, CallDirect, r1: Reg8, p1: UInt8, p2: FunctionIDUInt16),
  (83, Call2, r1: Reg8, r2: Reg8, r3: Reg8, r4: Reg8),
  (84, Call3, r1: Reg8, r2: Reg8, r3: Reg8, r4: Reg8, r5: Reg8),
  (85, Call4, r1: Reg8, r2: Reg8, r3: Reg8, r4: Reg8, r5: Reg8, r6: Reg8),
  (86, CallLong, r1: Reg8, r2: Reg8, p1: UInt32),
  (87, ConstructLong, r1: Reg8, r2: Reg8, p1: UInt32),
  (88, CallDirectLongIndex, r1: Reg8, p1: UInt8, p2: UInt32),
  (89, CallBuiltin, r1: Reg8, p1: UInt8, p2: UInt8),
  (90, CallBuiltinLong, r1: Reg8, p1: UInt8, p2: UInt32),
  (91, GetBuiltinClosure, r1: Reg8, p1: UInt8),
  (92, Ret, r1: Reg8),
  (93, Catch, r1: Reg8),
  (94, DirectEval, r1: Reg8, r2: Reg8, p1: UInt8),
  (95, Throw, r1: Reg8),
  (96, ThrowIfEmpty, r1: Reg8, r2: Reg8),
  (97, Debugger, ),
  (98, AsyncBreakCheck, ),
  (99, ProfilePoint, p1: UInt16),
  (100, CreateClosure, r1: Reg8, r2: Reg8, p1: FunctionIDUInt16),
  (101, CreateClosureLongIndex, r1: Reg8, r2: Reg8, p1: FunctionIDUInt32),
  (102, CreateGeneratorClosure, r1: Reg8, r2: Reg8, p1: FunctionIDUInt16),
  (103, CreateGeneratorClosureLongIndex, r1: Reg8, r2: Reg8, p1: FunctionIDUInt32),
  (104, CreateAsyncClosure, r1: Reg8, r2: Reg8, p1: FunctionIDUInt16),
  (105, CreateAsyncClosureLongIndex, r1: Reg8, r2: Reg8, p1: FunctionIDUInt32),
  (106, CreateThis, r1: Reg8, r2: Reg8, r3: Reg8),
  (107, SelectObject, r1: Reg8, r2: Reg8, r3: Reg8),
  (108, LoadParam, r1: Reg8, p1: UInt8),
  (109, LoadParamLong, r1: Reg8, p1: UInt32),
  (110, LoadConstUInt8, r1: Reg8, p1: UInt8),
  (111, LoadConstInt, r1: Reg8, p1: Imm32),
  (112, LoadConstDouble, r1: Reg8, p1: Double),
  (113, LoadConstBigInt, r1: Reg8, p1: BigIntIDUInt16),
  (114, LoadConstBigIntLongIndex, r1: Reg8, p1: BigIntIDUInt32),
  (115, LoadConstString, r1: Reg8, p1: StringIDUInt16),
  (116, LoadConstStringLongIndex, r1: Reg8, p1: StringIDUInt32),
  (117, LoadConstEmpty, r1: Reg8),
  (118, LoadConstUndefined, r1: Reg8),
  (119, LoadConstNull, r1: Reg8),
  (120, LoadConstTrue, r1: Reg8),
  (121, LoadConstFalse, r1: Reg8),
  (122, LoadConstZero, r1: Reg8),
  (123, CoerceThisNS, r1: Reg8, r2: Reg8),
  (124, LoadThisNS, r1: Reg8),
  (125, ToNumber, r1: Reg8, r2: Reg8),
  (126, ToNumeric, r1: Reg8, r2: Reg8),
  (127, ToInt32, r1: Reg8, r2: Reg8),
  (128, AddEmptyString, r1: Reg8, r2: Reg8),
  (129, GetArgumentsPropByVal, r1: Reg8, r2: Reg8, r3: Reg8),
  (130, GetArgumentsLength, r1: Reg8, r2: Reg8),
  (131, ReifyArguments, r1: Reg8),
  (132, CreateRegExp, r1: Reg8, p1: StringIDUInt32, p2: StringIDUInt32, p3: UInt32),
  (133, SwitchImm, r1: Reg8, p1: UInt32, p2: Addr32, p3: UInt32, p4: UInt32),
  (134, StartGenerator, ),
  (135, ResumeGenerator, r1: Reg8, r2: Reg8),
  (136, CompleteGenerator, ),
  (137, CreateGenerator, r1: Reg8, r2: Reg8, p1: FunctionIDUInt16),
  (138, CreateGeneratorLongIndex, r1: Reg8, r2: Reg8, p1: FunctionIDUInt32),
  (139, IteratorBegin, r1: Reg8, r2: Reg8),
  (140, IteratorNext, r1: Reg8, r2: Reg8, r3: Reg8),
  (141, IteratorClose, r1: Reg8, p1: UInt8),
  (142, Jmp, p1: Addr8),
  (143, JmpLong, p1: Addr32),
  (144, JmpTrue, p1: Addr8, r1: Reg8),
  (145, JmpTrueLong, p1: Addr32, r1: Reg8),
  (146, JmpFalse, p1: Addr8, r1: Reg8),
  (147, JmpFalseLong, p1: Addr32, r1: Reg8),
  (148, JmpUndefined, p1: Addr8, r1: Reg8),
  (149, JmpUndefinedLong, p1: Addr32, r1: Reg8),
  (150, SaveGenerator, p1: Addr8),
  (151, SaveGeneratorLong, p1: Addr32),
  (152, JLess, p1: Addr8, r1: Reg8, r2: Reg8),
  (153, JLessLong, p1: Addr32, r1: Reg8, r2: Reg8),
  (154, JNotLess, p1: Addr8, r1: Reg8, r2: Reg8),
  (155, JNotLessLong, p1: Addr32, r1: Reg8, r2: Reg8),
  (156, JLessN, p1: Addr8, r1: Reg8, r2: Reg8),
  (157, JLessNLong, p1: Addr32, r1: Reg8, r2: Reg8),
  (158, JNotLessN, p1: Addr8, r1: Reg8, r2: Reg8),
  (159, JNotLessNLong, p1: Addr32, r1: Reg8, r2: Reg8),
  (160, JLessEqual, p1: Addr8, r1: Reg8, r2: Reg8),
  (161, JLessEqualLong, p1: Addr32, r1: Reg8, r2: Reg8),
  (162, JNotLessEqual, p1: Addr8, r1: Reg8, r2: Reg8),
  (163, JNotLessEqualLong, p1: Addr32, r1: Reg8, r2: Reg8),
  (164, JLessEqualN, p1: Addr8, r1: Reg8, r2: Reg8),
  (165, JLessEqualNLong, p1: Addr32, r1: Reg8, r2: Reg8),
  (166, JNotLessEqualN, p1: Addr8, r1: Reg8, r2: Reg8),
  (167, JNotLessEqualNLong, p1: Addr32, r1: Reg8, r2: Reg8),
  (168, JGreater, p1: Addr8, r1: Reg8, r2: Reg8),
  (169, JGreaterLong, p1: Addr32, r1: Reg8, r2: Reg8),
  (170, JNotGreater, p1: Addr8, r1: Reg8, r2: Reg8),
  (171, JNotGreaterLong, p1: Addr32, r1: Reg8, r2: Reg8),
  (172, JGreaterN, p1: Addr8, r1: Reg8, r2: Reg8),
  (173, JGreaterNLong, p1: Addr32, r1: Reg8, r2: Reg8),
  (174, JNotGreaterN, p1: Addr8, r1: Reg8, r2: Reg8),
  (175, JNotGreaterNLong, p1: Addr32, r1: Reg8, r2: Reg8),
  (176, JGreaterEqual, p1: Addr8, r1: Reg8, r2: Reg8),
  (177, JGreaterEqualLong, p1: Addr32, r1: Reg8, r2: Reg8),
  (178, JNotGreaterEqual, p1: Addr8, r1: Reg8, r2: Reg8),
  (179, JNotGreaterEqualLong, p1: Addr32, r1: Reg8, r2: Reg8),
  (180, JGreaterEqualN, p1: Addr8, r1: Reg8, r2: Reg8),
  (181, JGreaterEqualNLong, p1: Addr32, r1: Reg8, r2: Reg8),
  (182, JNotGreaterEqualN, p1: Addr8, r1: Reg8, r2: Reg8),
  (183, JNotGreaterEqualNLong, p1: Addr32, r1: Reg8, r2: Reg8),
  (184, JEqual, p1: Addr8, r1: Reg8, r2: Reg8),
  (185, JEqualLong, p1: Addr32, r1: Reg8, r2: Reg8),
  (186, JNotEqual, p1: Addr8, r1: Reg8, r2: Reg8),
  (187, JNotEqualLong, p1: Addr32, r1: Reg8, r2: Reg8),
  (188, JStrictEqual, p1: Addr8, r1: Reg8, r2: Reg8),
  (189, JStrictEqualLong, p1: Addr32, r1: Reg8, r2: Reg8),
  (190, JStrictNotEqual, p1: Addr8, r1: Reg8, r2: Reg8),
  (191, JStrictNotEqualLong, p1: Addr32, r1: Reg8, r2: Reg8),
  (192, Add32, r1: Reg8, r2: Reg8, r3: Reg8),
  (193, Sub32, r1: Reg8, r2: Reg8, r3: Reg8),
  (194, Mul32, r1: Reg8, r2: Reg8, r3: Reg8),
  (195, Divi32, r1: Reg8, r2: Reg8, r3: Reg8),
  (196, Divu32, r1: Reg8, r2: Reg8, r3: Reg8),
  (197, Loadi8, r1: Reg8, r2: Reg8, r3: Reg8),
  (198, Loadu8, r1: Reg8, r2: Reg8, r3: Reg8),
  (199, Loadi16, r1: Reg8, r2: Reg8, r3: Reg8),
  (200, Loadu16, r1: Reg8, r2: Reg8, r3: Reg8),
  (201, Loadi32, r1: Reg8, r2: Reg8, r3: Reg8),
  (202, Loadu32, r1: Reg8, r2: Reg8, r3: Reg8),
  (203, Store8, r1: Reg8, r2: Reg8, r3: Reg8),
  (204, Store16, r1: Reg8, r2: Reg8, r3: Reg8),
  (205, Store32, r1: Reg8, r2: Reg8, r3: Reg8)
);
