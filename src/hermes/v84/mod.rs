use crate::hermes;

build_instructions!(
  (0, Unreachable, ),
  (1, NewObjectWithBuffer, r0: Reg8, p0: UInt16, p1: UInt16, p2: UInt16, p3: UInt16),
  (2, NewObjectWithBufferLong, r0: Reg8, p0: UInt16, p1: UInt16, p2: UInt32, p3: UInt32),
  (3, NewObject, r0: Reg8),
  (4, NewObjectWithParent, r0: Reg8, r1: Reg8),
  (5, NewArrayWithBuffer, r0: Reg8, p0: UInt16, p1: UInt16, p2: UInt16),
  (6, NewArrayWithBufferLong, r0: Reg8, p0: UInt16, p1: UInt16, p2: UInt32),
  (7, NewArray, r0: Reg8, p0: UInt16),
  (8, Mov, r0: Reg8, r1: Reg8),
  (9, MovLong, r0: Reg32, r1: Reg32),
  (10, Negate, r0: Reg8, r1: Reg8),
  (11, Not, r0: Reg8, r1: Reg8),
  (12, BitNot, r0: Reg8, r1: Reg8),
  (13, TypeOf, r0: Reg8, r1: Reg8),
  (14, Eq, r0: Reg8, r1: Reg8, r2: Reg8),
  (15, StrictEq, r0: Reg8, r1: Reg8, r2: Reg8),
  (16, Neq, r0: Reg8, r1: Reg8, r2: Reg8),
  (17, StrictNeq, r0: Reg8, r1: Reg8, r2: Reg8),
  (18, Less, r0: Reg8, r1: Reg8, r2: Reg8),
  (19, LessEq, r0: Reg8, r1: Reg8, r2: Reg8),
  (20, Greater, r0: Reg8, r1: Reg8, r2: Reg8),
  (21, GreaterEq, r0: Reg8, r1: Reg8, r2: Reg8),
  (22, Add, r0: Reg8, r1: Reg8, r2: Reg8),
  (23, AddN, r0: Reg8, r1: Reg8, r2: Reg8),
  (24, Mul, r0: Reg8, r1: Reg8, r2: Reg8),
  (25, MulN, r0: Reg8, r1: Reg8, r2: Reg8),
  (26, Div, r0: Reg8, r1: Reg8, r2: Reg8),
  (27, DivN, r0: Reg8, r1: Reg8, r2: Reg8),
  (28, Mod, r0: Reg8, r1: Reg8, r2: Reg8),
  (29, Sub, r0: Reg8, r1: Reg8, r2: Reg8),
  (30, SubN, r0: Reg8, r1: Reg8, r2: Reg8),
  (31, LShift, r0: Reg8, r1: Reg8, r2: Reg8),
  (32, RShift, r0: Reg8, r1: Reg8, r2: Reg8),
  (33, URshift, r0: Reg8, r1: Reg8, r2: Reg8),
  (34, BitAnd, r0: Reg8, r1: Reg8, r2: Reg8),
  (35, BitXor, r0: Reg8, r1: Reg8, r2: Reg8),
  (36, BitOr, r0: Reg8, r1: Reg8, r2: Reg8),
  (37, InstanceOf, r0: Reg8, r1: Reg8, r2: Reg8),
  (38, IsIn, r0: Reg8, r1: Reg8, r2: Reg8),
  (39, GetEnvironment, r0: Reg8, p0: UInt8),
  (40, StoreToEnvironment, r0: Reg8, p0: UInt8, r1: Reg8),
  (41, StoreToEnvironmentL, r0: Reg8, p0: UInt16, r1: Reg8),
  (42, StoreNPToEnvironment, r0: Reg8, p0: UInt8, r1: Reg8),
  (43, StoreNPToEnvironmentL, r0: Reg8, p0: UInt16, r1: Reg8),
  (44, LoadFromEnvironment, r0: Reg8, r1: Reg8, p0: UInt8),
  (45, LoadFromEnvironmentL, r0: Reg8, r1: Reg8, p0: UInt16),
  (46, GetGlobalObject, r0: Reg8),
  (47, GetNewTarget, r0: Reg8),
  (48, CreateEnvironment, r0: Reg8),
  (49, DeclareGlobalVar, p0: StringIDUInt32),
  (50, GetByIdShort, r0: Reg8, r1: Reg8, p0: UInt8, p1: StringIDUInt8),
  (51, GetById, r0: Reg8, r1: Reg8, p0: UInt8, p1: StringIDUInt16),
  (52, GetByIdLong, r0: Reg8, r1: Reg8, p0: UInt8, p1: StringIDUInt32),
  (53, TryGetById, r0: Reg8, r1: Reg8, p0: UInt8, p1: StringIDUInt16),
  (54, TryGetByIdLong, r0: Reg8, r1: Reg8, p0: UInt8, p1: StringIDUInt32),
  (55, PutById, r0: Reg8, r1: Reg8, p0: UInt8, p1: StringIDUInt16),
  (56, PutByIdLong, r0: Reg8, r1: Reg8, p0: UInt8, p1: StringIDUInt32),
  (57, TryPutById, r0: Reg8, r1: Reg8, p0: UInt8, p1: StringIDUInt16),
  (58, TryPutByIdLong, r0: Reg8, r1: Reg8, p0: UInt8, p1: StringIDUInt32),
  (59, PutNewOwnByIdShort, r0: Reg8, r1: Reg8, p0: StringIDUInt8),
  (60, PutNewOwnById, r0: Reg8, r1: Reg8, p0: StringIDUInt16),
  (61, PutNewOwnByIdLong, r0: Reg8, r1: Reg8, p0: StringIDUInt32),
  (62, PutNewOwnNEById, r0: Reg8, r1: Reg8, p0: StringIDUInt16),
  (63, PutNewOwnNEByIdLong, r0: Reg8, r1: Reg8, p0: StringIDUInt32),
  (64, PutOwnByIndex, r0: Reg8, r1: Reg8, p0: UInt8),
  (65, PutOwnByIndexL, r0: Reg8, r1: Reg8, p0: UInt32),
  (66, PutOwnByVal, r0: Reg8, r1: Reg8, r2: Reg8, p0: UInt8),
  (67, DelById, r0: Reg8, r1: Reg8, p0: StringIDUInt16),
  (68, DelByIdLong, r0: Reg8, r1: Reg8, p0: StringIDUInt32),
  (69, GetByVal, r0: Reg8, r1: Reg8, r2: Reg8),
  (70, PutByVal, r0: Reg8, r1: Reg8, r2: Reg8),
  (71, DelByVal, r0: Reg8, r1: Reg8, r2: Reg8),
  (72, PutOwnGetterSetterByVal, r0: Reg8, r1: Reg8, r2: Reg8, r3: Reg8, p0: UInt8),
  (73, GetPNameList, r0: Reg8, r1: Reg8, r2: Reg8, r3: Reg8),
  (74, GetNextPName, r0: Reg8, r1: Reg8, r2: Reg8, r3: Reg8, r4: Reg8),
  (75, Call, r0: Reg8, r1: Reg8, p0: UInt8),
  (76, Construct, r0: Reg8, r1: Reg8, p0: UInt8),
  (77, Call1, r0: Reg8, r1: Reg8, r2: Reg8),
  (78, CallDirect, r0: Reg8, p0: UInt8, p1: UInt16),
  (79, Call2, r0: Reg8, r1: Reg8, r2: Reg8, r3: Reg8),
  (80, Call3, r0: Reg8, r1: Reg8, r2: Reg8, r3: Reg8, r4: Reg8),
  (81, Call4, r0: Reg8, r1: Reg8, r2: Reg8, r3: Reg8, r4: Reg8, r5: Reg8),
  (82, CallLong, r0: Reg8, r1: Reg8, p0: UInt32),
  (83, ConstructLong, r0: Reg8, r1: Reg8, p0: UInt32),
  (84, CallDirectLongIndex, r0: Reg8, p0: UInt8, p1: UInt32),
  (85, CallBuiltin, r0: Reg8, p0: UInt8, p1: UInt8),
  (86, CallBuiltinLong, r0: Reg8, p0: UInt8, p1: UInt32),
  (87, GetBuiltinClosure, r0: Reg8, p0: UInt8),
  (88, Ret, r0: Reg8),
  (89, Catch, r0: Reg8),
  (90, DirectEval, r0: Reg8, r1: Reg8),
  (91, Throw, r0: Reg8),
  (92, ThrowIfEmpty, r0: Reg8, r1: Reg8),
  (93, Debugger, ),
  (94, AsyncBreakCheck, ),
  (95, ProfilePoint, p0: UInt16),
  (96, CreateClosure, r0: Reg8, r1: Reg8, p0: UInt16),
  (97, CreateClosureLongIndex, r0: Reg8, r1: Reg8, p0: UInt32),
  (98, CreateGeneratorClosure, r0: Reg8, r1: Reg8, p0: UInt16),
  (99, CreateGeneratorClosureLongIndex, r0: Reg8, r1: Reg8, p0: UInt32),
  (100, CreateAsyncClosure, r0: Reg8, r1: Reg8, p0: UInt16),
  (101, CreateAsyncClosureLongIndex, r0: Reg8, r1: Reg8, p0: UInt32),
  (102, CreateThis, r0: Reg8, r1: Reg8, r2: Reg8),
  (103, SelectObject, r0: Reg8, r1: Reg8, r2: Reg8),
  (104, LoadParam, r0: Reg8, p0: UInt8),
  (105, LoadParamLong, r0: Reg8, p0: UInt32),
  (106, LoadConstUInt8, r0: Reg8, p0: UInt8),
  (107, LoadConstInt, r0: Reg8, p0: Imm32),
  (108, LoadConstDouble, r0: Reg8, p0: Double),
  (109, LoadConstString, r0: Reg8, p0: StringIDUInt16),
  (110, LoadConstStringLongIndex, r0: Reg8, p0: StringIDUInt32),
  (111, LoadConstEmpty, r0: Reg8),
  (112, LoadConstUndefined, r0: Reg8),
  (113, LoadConstNull, r0: Reg8),
  (114, LoadConstTrue, r0: Reg8),
  (115, LoadConstFalse, r0: Reg8),
  (116, LoadConstZero, r0: Reg8),
  (117, CoerceThisNS, r0: Reg8, r1: Reg8),
  (118, LoadThisNS, r0: Reg8),
  (119, ToNumber, r0: Reg8, r1: Reg8),
  (120, ToInt32, r0: Reg8, r1: Reg8),
  (121, AddEmptyString, r0: Reg8, r1: Reg8),
  (122, GetArgumentsPropByVal, r0: Reg8, r1: Reg8, r2: Reg8),
  (123, GetArgumentsLength, r0: Reg8, r1: Reg8),
  (124, ReifyArguments, r0: Reg8),
  (125, CreateRegExp, r0: Reg8, p0: StringIDUInt32, p1: StringIDUInt32, p2: UInt32),
  (126, SwitchImm, r0: Reg8, p0: UInt32, p1: Addr32, p2: UInt32, p3: UInt32),
  (127, StartGenerator, ),
  (128, ResumeGenerator, r0: Reg8, r1: Reg8),
  (129, CompleteGenerator, ),
  (130, CreateGenerator, r0: Reg8, r1: Reg8, p0: UInt16),
  (131, CreateGeneratorLongIndex, r0: Reg8, r1: Reg8, p0: UInt32),
  (132, IteratorBegin, r0: Reg8, r1: Reg8),
  (133, IteratorNext, r0: Reg8, r1: Reg8, r2: Reg8),
  (134, IteratorClose, r0: Reg8, p0: UInt8),
  (135, Jmp, p0: Addr8),
  (136, JmpLong, p0: Addr32),
  (137, JmpTrue, p0: Addr8, r0: Reg8),
  (138, JmpTrueLong, p0: Addr32, r0: Reg8),
  (139, JmpFalse, p0: Addr8, r0: Reg8),
  (140, JmpFalseLong, p0: Addr32, r0: Reg8),
  (141, JmpUndefined, p0: Addr8, r0: Reg8),
  (142, JmpUndefinedLong, p0: Addr32, r0: Reg8),
  (143, SaveGenerator, p0: Addr8),
  (144, SaveGeneratorLong, p0: Addr32),
  (145, JLess, p0: Addr8, r0: Reg8, r1: Reg8),
  (146, JLessLong, p0: Addr32, r0: Reg8, r1: Reg8),
  (147, JNotLess, p0: Addr8, r0: Reg8, r1: Reg8),
  (148, JNotLessLong, p0: Addr32, r0: Reg8, r1: Reg8),
  (149, JLessN, p0: Addr8, r0: Reg8, r1: Reg8),
  (150, JLessNLong, p0: Addr32, r0: Reg8, r1: Reg8),
  (151, JNotLessN, p0: Addr8, r0: Reg8, r1: Reg8),
  (152, JNotLessNLong, p0: Addr32, r0: Reg8, r1: Reg8),
  (153, JLessEqual, p0: Addr8, r0: Reg8, r1: Reg8),
  (154, JLessEqualLong, p0: Addr32, r0: Reg8, r1: Reg8),
  (155, JNotLessEqual, p0: Addr8, r0: Reg8, r1: Reg8),
  (156, JNotLessEqualLong, p0: Addr32, r0: Reg8, r1: Reg8),
  (157, JLessEqualN, p0: Addr8, r0: Reg8, r1: Reg8),
  (158, JLessEqualNLong, p0: Addr32, r0: Reg8, r1: Reg8),
  (159, JNotLessEqualN, p0: Addr8, r0: Reg8, r1: Reg8),
  (160, JNotLessEqualNLong, p0: Addr32, r0: Reg8, r1: Reg8),
  (161, JGreater, p0: Addr8, r0: Reg8, r1: Reg8),
  (162, JGreaterLong, p0: Addr32, r0: Reg8, r1: Reg8),
  (163, JNotGreater, p0: Addr8, r0: Reg8, r1: Reg8),
  (164, JNotGreaterLong, p0: Addr32, r0: Reg8, r1: Reg8),
  (165, JGreaterN, p0: Addr8, r0: Reg8, r1: Reg8),
  (166, JGreaterNLong, p0: Addr32, r0: Reg8, r1: Reg8),
  (167, JNotGreaterN, p0: Addr8, r0: Reg8, r1: Reg8),
  (168, JNotGreaterNLong, p0: Addr32, r0: Reg8, r1: Reg8),
  (169, JGreaterEqual, p0: Addr8, r0: Reg8, r1: Reg8),
  (170, JGreaterEqualLong, p0: Addr32, r0: Reg8, r1: Reg8),
  (171, JNotGreaterEqual, p0: Addr8, r0: Reg8, r1: Reg8),
  (172, JNotGreaterEqualLong, p0: Addr32, r0: Reg8, r1: Reg8),
  (173, JGreaterEqualN, p0: Addr8, r0: Reg8, r1: Reg8),
  (174, JGreaterEqualNLong, p0: Addr32, r0: Reg8, r1: Reg8),
  (175, JNotGreaterEqualN, p0: Addr8, r0: Reg8, r1: Reg8),
  (176, JNotGreaterEqualNLong, p0: Addr32, r0: Reg8, r1: Reg8),
  (177, JEqual, p0: Addr8, r0: Reg8, r1: Reg8),
  (178, JEqualLong, p0: Addr32, r0: Reg8, r1: Reg8),
  (179, JNotEqual, p0: Addr8, r0: Reg8, r1: Reg8),
  (180, JNotEqualLong, p0: Addr32, r0: Reg8, r1: Reg8),
  (181, JStrictEqual, p0: Addr8, r0: Reg8, r1: Reg8),
  (182, JStrictEqualLong, p0: Addr32, r0: Reg8, r1: Reg8),
  (183, JStrictNotEqual, p0: Addr8, r0: Reg8, r1: Reg8),
  (184, JStrictNotEqualLong, p0: Addr32, r0: Reg8, r1: Reg8),
  (185, Add32, r0: Reg8, r1: Reg8, r2: Reg8),
  (186, Sub32, r0: Reg8, r1: Reg8, r2: Reg8),
  (187, Mul32, r0: Reg8, r1: Reg8, r2: Reg8),
  (188, Divi32, r0: Reg8, r1: Reg8, r2: Reg8),
  (189, Divu32, r0: Reg8, r1: Reg8, r2: Reg8),
  (190, Loadi8, r0: Reg8, r1: Reg8, r2: Reg8),
  (191, Loadu8, r0: Reg8, r1: Reg8, r2: Reg8),
  (192, Loadi16, r0: Reg8, r1: Reg8, r2: Reg8),
  (193, Loadu16, r0: Reg8, r1: Reg8, r2: Reg8),
  (194, Loadi32, r0: Reg8, r1: Reg8, r2: Reg8),
  (195, Loadu32, r0: Reg8, r1: Reg8, r2: Reg8),
  (196, Store8, r0: Reg8, r1: Reg8, r2: Reg8),
  (197, Store16, r0: Reg8, r1: Reg8, r2: Reg8),
  (198, Store32, r0: Reg8, r1: Reg8, r2: Reg8)
);