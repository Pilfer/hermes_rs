use crate::hermes;

build_instructions!(
    /// Unreachable opcode for stubs and similar. This is first so that it has the
    /// value zero.
    (0, Unreachable, ),
    /// Create an object from a static map of values, as for var={'a': 3}.
    /// Any non-constant elements can be set afterwards with PutOwnByInd.
    /// Arg1 is the destination.
    /// Arg2 is a preallocation size hint.
    /// Arg3 is the number of static elements.
    /// Arg4 is the index in the object key buffer table.
    /// Arg5 is the index in the object val buffer table.
    (1, NewObjectWithBuffer, r0: Reg8, p0: UInt16, p1: UInt16, p2: UInt16, p3: UInt16),
    /// Create an object from a static map of values, as for var={'a': 3}.
    /// Any non-constant elements can be set afterwards with PutOwnByInd.
    /// Arg1 is the destination.
    /// Arg2 is a preallocation size hint.
    /// Arg3 is the number of static elements.
    /// Arg4 is the index in the object key buffer table.
    /// Arg5 is the index in the object val buffer table.
    (2, NewObjectWithBufferLong, r0: Reg8, p0: UInt16, p1: UInt16, p2: UInt32, p3: UInt32),
    /// Create a new, empty Object using the built-in constructor (regardless of /// whether it was overridden).
    /// Arg1 = {}
    (3, NewObject, r0: Reg8),
    /// Create a new empty Object with the specified parent. If the parent is
    /// null, no parent is used. If the parent is not an object, the builtin
    /// Object.prototype is used. Otherwise the parent itself is used.
    /// Arg1 = the created object
    /// Arg2 = the parent.
    (4, NewObjectWithParent, r0: Reg8, r1: Reg8),
    /// Create an array from a static list of values, as for var=[1,2,3].
    /// Any non-constant elements can be set afterwards with PutOwnByIndex.
    /// Arg1 is the destination.
    /// Arg2 is a preallocation size hint.
    /// Arg3 is the number of static elements.
    /// Arg4 is the index in the array buffer table.
    (5, NewArrayWithBuffer, r0: Reg8, p0: UInt16, p1: UInt16, p2: UInt16),
    /// Create an array from a static list of values, as for var=[1,2,3].
    /// Any non-constant elements can be set afterwards with PutOwnByIndex.
    /// Arg1 is the destination.
    /// Arg2 is a preallocation size hint.
    /// Arg3 is the number of static elements.
    /// Arg4 is the index in the array buffer table.
    (6, NewArrayWithBufferLong, r0: Reg8, p0: UInt16, p1: UInt16, p2: UInt32),
    /// Create a new array of a given size.
    /// Arg1 = new Array(Arg2)
    (7, NewArray, r0: Reg8, p0: UInt16),
    /// Arg1 = Arg2 (Register copy)
    (8, Mov, r0: Reg8, r1: Reg8),
    /// Arg1 = Arg2 (Register copy, long index)
    (9, MovLong, r0: Reg32, r1: Reg32),
    /// Arg1 = -Arg2 (Unary minus)
    (10, Negate, r0: Reg8, r1: Reg8),
    /// Arg1 = !Arg2 (Boolean not)
    (11, Not, r0: Reg8, r1: Reg8),
    /// Arg1 = ~Arg2 (Bitwise not)
    (12, BitNot, r0: Reg8, r1: Reg8),
    /// Arg1 = typeof Arg2 (JS typeof)
    (13, TypeOf, r0: Reg8, r1: Reg8),
    /// Arg1 = Arg2 == Arg3 (JS equality)
    (14, Eq, r0: Reg8, r1: Reg8, r2: Reg8),
    /// Arg1 = Arg2 === Arg3 (JS strict equality)
    (15, StrictEq, r0: Reg8, r1: Reg8, r2: Reg8),
    /// Arg1 = Arg2 != Arg3 (JS inequality)
    (16, Neq, r0: Reg8, r1: Reg8, r2: Reg8),
    /// Arg1 = Arg2 !== Arg3 (JS strict inequality)
    (17, StrictNeq, r0: Reg8, r1: Reg8, r2: Reg8),
    /// Arg1 = Arg2 < Arg3 (JS less-than)
    (18, Less, r0: Reg8, r1: Reg8, r2: Reg8),
    /// Arg1 = Arg2 <= Arg3 (JS less-than-or-equals)
    (19, LessEq, r0: Reg8, r1: Reg8, r2: Reg8),
    /// Arg1 = Arg2 > Arg3 (JS greater-than)
    (20, Greater, r0: Reg8, r1: Reg8, r2: Reg8),
    /// Arg1 = Arg2 >= Arg3 (JS greater-than-or-equals)
    (21, GreaterEq, r0: Reg8, r1: Reg8, r2: Reg8),
    /// Arg1 = Arg2 + Arg3 (JS addition/concatenation)
    (22, Add, r0: Reg8, r1: Reg8, r2: Reg8),
    /// Arg1 = Arg2 + Arg3 (Numeric addition, skips number check)
    (23, AddN, r0: Reg8, r1: Reg8, r2: Reg8),
    /// Arg1 = Arg2 * Arg3 (JS multiplication)
    (24, Mul, r0: Reg8, r1: Reg8, r2: Reg8),
    /// Arg1 = Arg2 * Arg3 (Numeric multiplication, skips number check)
    (25, MulN, r0: Reg8, r1: Reg8, r2: Reg8),
    /// Arg1 = Arg2 / Arg3 (JS division)
    (26, Div, r0: Reg8, r1: Reg8, r2: Reg8),
    /// Arg1 = Arg2 / Arg3 (Numeric division, skips number check)
    (27, DivN, r0: Reg8, r1: Reg8, r2: Reg8),
    /// Arg1 = Arg2 % Arg3 (JS remainder)
    (28, Mod, r0: Reg8, r1: Reg8, r2: Reg8),
    /// Arg1 = Arg2 - Arg3 (JS subtraction)
    (29, Sub, r0: Reg8, r1: Reg8, r2: Reg8),
    /// Arg1 = Arg2 - Arg3 (Numeric subtraction, skips number check)
    (30, SubN, r0: Reg8, r1: Reg8, r2: Reg8),
    /// Arg1 = Arg2 << Arg3 (JS bitshift left)
    (31, LShift, r0: Reg8, r1: Reg8, r2: Reg8),
    /// Arg1 = Arg2 >> Arg3 (JS signed bitshift right)
    (32, RShift, r0: Reg8, r1: Reg8, r2: Reg8),
    /// Arg1 = Arg2 >>> Arg3 (JS unsigned bitshift right)
    (33, URshift, r0: Reg8, r1: Reg8, r2: Reg8),
    /// Arg1 = Arg2 & Arg3 (JS bitwise AND)
    (34, BitAnd, r0: Reg8, r1: Reg8, r2: Reg8),
    /// Arg1 = Arg2 ^ Arg3 (JS bitwise XOR)
    (35, BitXor, r0: Reg8, r1: Reg8, r2: Reg8),
    /// Arg1 = Arg2 | Arg3 (JS bitwise OR)
    (36, BitOr, r0: Reg8, r1: Reg8, r2: Reg8),
    /// Check whether Arg2 contains Arg3 in its prototype chain.
    /// Note that this is not the same as JS instanceof.
    /// Pseudocode: Arg1 = prototypechain(Arg2).contains(Arg3)
    (37, InstanceOf, r0: Reg8, r1: Reg8, r2: Reg8),
    /// Arg1 = Arg2 in Arg3 (JS relational 'in')
    (38, IsIn, r0: Reg8, r1: Reg8, r2: Reg8),
    /// Get an environment (scope) from N levels up the stack.
    /// 0 is the current environment, 1 is the caller's environment, etc.
    (39, GetEnvironment, r0: Reg8, p0: UInt8),
    /// Store a value in an environment.
    /// StoreNPToEnvironment[L] store a non-pointer value in an environment
    /// Arg1 is the environment (as fetched by GetEnvironment).
    /// Arg2 is the environment index slot number.
    /// Arg3 is the value.
    (40, StoreToEnvironment, r0: Reg8, p0: UInt8, r1: Reg8),
    /// Store a value in an environment.
    /// StoreNPToEnvironment[L] store a non-pointer value in an environment
    /// Arg1 is the environment (as fetched by GetEnvironment).
    /// Arg2 is the environment index slot number.
    /// Arg3 is the value.
    (41, StoreToEnvironmentL, r0: Reg8, p0: UInt16, r1: Reg8),
    /// Store a value in an environment.
    /// StoreNPToEnvironment[L] store a non-pointer value in an environment
    /// Arg1 is the environment (as fetched by GetEnvironment).
    /// Arg2 is the environment index slot number.
    /// Arg3 is the value.
    (42, StoreNPToEnvironment, r0: Reg8, p0: UInt8, r1: Reg8),
    /// Store a value in an environment.
    /// StoreNPToEnvironment[L] store a non-pointer value in an environment
    /// Arg1 is the environment (as fetched by GetEnvironment).
    /// Arg2 is the environment index slot number.
    /// Arg3 is the value.
    (43, StoreNPToEnvironmentL, r0: Reg8, p0: UInt16, r1: Reg8),
    /// Load a value from an environment.
    /// Arg1 is the destination.
    /// Arg2 is the environment (as fetched by GetEnvironment).
    /// Arg3 is the environment index slot number.
    (44, LoadFromEnvironment, r0: Reg8, r1: Reg8, p0: UInt8),
    /// Load a value from an environment.
    /// Arg1 is the destination.
    /// Arg2 is the environment (as fetched by GetEnvironment).
    /// Arg3 is the environment index slot number.
    (45, LoadFromEnvironmentL, r0: Reg8, r1: Reg8, p0: UInt16),
    /// Get the global object (the object in which global variables are stored).
    (46, GetGlobalObject, r0: Reg8),
    /// Obtain the value of NewTarget from the frame.
    /// Arg1 = NewTarget
    (47, GetNewTarget, r0: Reg8),
    /// Create a new environment, to store values captured by closures.
    (48, CreateEnvironment, r0: Reg8),
    /// Declare a global variable by string table index.
    /// The variable will be set to undefined.
    (49, DeclareGlobalVar, p0: StringIDUInt32),
    /// Get an object property by string table index.
    /// Arg1 = Arg2[stringtable[Arg4]]
    /// Arg3 is a cache index used to speed up the above operation.
    (50, GetByIdShort, r0: Reg8, r1: Reg8, p0: UInt8, p1: StringIDUInt8),
    /// Get an object property by string table index.
    /// Arg1 = Arg2[stringtable[Arg4]]
    /// Arg3 is a cache index used to speed up the above operation.
    (51, GetById, r0: Reg8, r1: Reg8, p0: UInt8, p1: StringIDUInt16),
    /// Get an object property by string table index.
    /// Arg1 = Arg2[stringtable[Arg4]]
    /// Arg3 is a cache index used to speed up the above operation.
    (52, GetByIdLong, r0: Reg8, r1: Reg8, p0: UInt8, p1: StringIDUInt32),
    /// Get an object property by string table index, or throw if not found.
    /// This is similar to GetById, but intended for use with global variables
    /// where Arg2 = GetGlobalObject.
    (53, TryGetById, r0: Reg8, r1: Reg8, p0: UInt8, p1: StringIDUInt16),
    /// Get an object property by string table index, or throw if not found.
    /// This is similar to GetById, but intended for use with global variables
    /// where Arg2 = GetGlobalObject.
    (54, TryGetByIdLong, r0: Reg8, r1: Reg8, p0: UInt8, p1: StringIDUInt32),
    /// Set an object property by string index.
    /// Arg1[stringtable[Arg4]] = Arg2.
    (55, PutById, r0: Reg8, r1: Reg8, p0: UInt8, p1: StringIDUInt16),
    /// Set an object property by string index.
    /// Arg1[stringtable[Arg4]] = Arg2.
    (56, PutByIdLong, r0: Reg8, r1: Reg8, p0: UInt8, p1: StringIDUInt32),
    /// Set an object property by string index, or throw if undeclared.
    /// This is similar to PutById, but intended for use with global variables
    /// where Arg1 = GetGlobalObject.
    (57, TryPutById, r0: Reg8, r1: Reg8, p0: UInt8, p1: StringIDUInt16),
    /// Set an object property by string index, or throw if undeclared.
    /// This is similar to PutById, but intended for use with global variables
    /// where Arg1 = GetGlobalObject.
    (58, TryPutByIdLong, r0: Reg8, r1: Reg8, p0: UInt8, p1: StringIDUInt32),
    /// Create a new own property on an object. This is similar to PutById, but
    /// the destination must be an object, it only deals with own properties,
    /// ignoring the prototype chain, and the property must not already be defined.
    /// Similarly to PutById, the property name cannot be a valid array index.
    /// Arg1 is the destination object, which is known to be an object.
    /// Arg2 is the value to write.
    /// Arg3 is the string table ID of the property name.
    /// Arg1[stringtable[Arg3]] = Arg2
    (59, PutNewOwnByIdShort, r0: Reg8, r1: Reg8, p0: StringIDUInt8),
    /// Create a new own property on an object. This is similar to PutById, but
    /// the destination must be an object, it only deals with own properties,
    /// ignoring the prototype chain, and the property must not already be defined.
    /// Similarly to PutById, the property name cannot be a valid array index.
    /// Arg1 is the destination object, which is known to be an object.
    /// Arg2 is the value to write.
    /// Arg3 is the string table ID of the property name.
    /// Arg1[stringtable[Arg3]] = Arg2
    (60, PutNewOwnById, r0: Reg8, r1: Reg8, p0: StringIDUInt16),
    /// Create a new own property on an object. This is similar to PutById, but
    /// the destination must be an object, it only deals with own properties,
    /// ignoring the prototype chain, and the property must not already be defined.
    /// Similarly to PutById, the property name cannot be a valid array index.
    /// Arg1 is the destination object, which is known to be an object.
    /// Arg2 is the value to write.
    /// Arg3 is the string table ID of the property name.
    /// Arg1[stringtable[Arg3]] = Arg2
    (61, PutNewOwnByIdLong, r0: Reg8, r1: Reg8, p0: StringIDUInt32),
    /// Create a new non-enumerable own property on an object. This is the same as
    /// PutNewOwnById, but creates the property with different enumerability.
    /// Arg1 is the destination object.
    /// Arg2 is the value to write.
    /// Arg3 is the string table ID of the property name.
    /// Arg1[stringtable[Arg3]] = Arg2
    (62, PutNewOwnNEById, r0: Reg8, r1: Reg8, p0: StringIDUInt16),
    /// Create a new non-enumerable own property on an object. This is the same as
    /// PutNewOwnById, but creates the property with different enumerability.
    /// Arg1 is the destination object.
    /// Arg2 is the value to write.
    /// Arg3 is the string table ID of the property name.
    /// Arg1[stringtable[Arg3]] = Arg2
    (63, PutNewOwnNEByIdLong, r0: Reg8, r1: Reg8, p0: StringIDUInt32),
    /// Assign a value to a constant integer own property which will be created as
    /// enumerable. This is used (potentially in conjunction with /// NewArrayWithBuffer) for arr=[foo,bar] initializations.
    /// Arg1[Arg3] = Arg2;
    (64, PutOwnByIndex, r0: Reg8, r1: Reg8, p0: UInt8),
    /// Assign a value to a constant integer own property which will be created as
    /// enumerable. This is used (potentially in conjunction with /// NewArrayWithBuffer) for arr=[foo,bar] initializations.
    /// Arg1[Arg3] = Arg2;
    (65, PutOwnByIndexL, r0: Reg8, r1: Reg8, p0: UInt32),
    /// Set an own property identified by value.
    /// Arg1 is the destination object.
    /// Arg2 is the value to write.
    /// Arg3 is the property name.
    /// Arg4 : bool -> enumerable. If true, the property is created as enumerable,
    ///non-enumerable otherwise.
    /// Arg1[Arg3] = Arg2;
    (66, PutOwnByVal, r0: Reg8, r1: Reg8, r2: Reg8, p0: UInt8),
    /// Delete a property by string table index.
    /// Arg1 = delete Arg2[stringtable[Arg3]]
    (67, DelById, r0: Reg8, r1: Reg8, p0: StringIDUInt16),
    /// Delete a property by string table index.
    /// Arg1 = delete Arg2[stringtable[Arg3]]
    (68, DelByIdLong, r0: Reg8, r1: Reg8, p0: StringIDUInt32),
    /// Get a property by value. Constants string values should instead use GetById.
    /// Arg1 = Arg2[Arg3]
    (69, GetByVal, r0: Reg8, r1: Reg8, r2: Reg8),
    /// Set a property by value. Constant string values should instead use GetById
    /// (unless they are array indices according to ES5.1 section 15.4, in which /// case this is still the right opcode).
    /// Arg1[Arg2] = Arg3
    (70, PutByVal, r0: Reg8, r1: Reg8, r2: Reg8),
    /// Delete a property by value (when the value is not known at compile time).
    /// Arg1 = delete Arg2[Arg3]
    (71, DelByVal, r0: Reg8, r1: Reg8, r2: Reg8),
    /// Add a getter and a setter for a property by value.
    /// Object.defineProperty(Arg1, Arg2, { get: Arg3, set: Arg4 }).
    /// Arg1 is the target object which will have a property defined.
    /// Arg2 is the property name
    /// Arg3 is the getter closure or undefined
    /// Arg4 is the setter closure or undefined
    /// Arg5 : boolean - if true, the property will be enumerable.
    (72, PutOwnGetterSetterByVal, r0: Reg8, r1: Reg8, r2: Reg8, r3: Reg8, p0: UInt8),
    /// Get the list of properties from an object to implement for..in loop.
    /// Returns Arg1, which is the register that holds array of properties.
    /// Returns Undefined if the object is null/undefined.
    /// Arg2 is the register that holds the object.
    /// Arg3 is the register that holds the iterating index.
    /// Arg4 is the register that holds the size of the property list.
    (73, GetPNameList, r0: Reg8, r1: Reg8, r2: Reg8, r3: Reg8),
    /// Get the next property in the for..in iterator.
    /// Returns Arg1, which is the next property. Undefined if unavailable.
    /// Arg2 is the register that holds array of properties.
    /// Arg3 is the register that holds the object.
    /// Arg4 is the register that holds the iterating index.
    /// Arg5 is the register that holds the size of the property list.
    (74, GetNextPName, r0: Reg8, r1: Reg8, r2: Reg8, r3: Reg8, r4: Reg8),
    /// NOTE: the ordering of Call, CallN, Construct, CallLong, ConstructLong is
    /// important. The "long" versions are defined after the "short" versions.
    /// It is important for efficiency that all Calls have contiguous opcodes, with
    /// wider instructions appearing after narrower ones.
    /// Call a function.
    /// Arg1 is the destination of the return value.
    /// Arg2 is the closure to invoke.
    /// Arg3 is the number of arguments, assumed to be found in reverse order
    ///from the end of the current frame.
    (75, Call, r0: Reg8, r1: Reg8, p0: UInt8),
    /// Call a constructor, with semantics identical to Call.
    /// Arg1 is the destination of the return value.
    /// Arg2 is the closure to invoke.
    /// Arg3 is the number of arguments, assumed to be found in reverse order
    ///from the end of the current frame. The first argument 'this'
    ///is assumed to be created with CreateThis.
    (76, Construct, r0: Reg8, r1: Reg8, p0: UInt8),
    /// Call a function with one arg.
    /// Arg1 is the destination of the return value.
    /// Arg2 is the closure to invoke.
    /// Arg3 is the first argument.
    (77, Call1, r0: Reg8, r1: Reg8, r2: Reg8),
    /// Call a function directly without a closure.
    /// Arg1 is the destination of the return value.
    /// Arg2 is the number of arguments, assumed to be found in reverse order
    ///from the end of the current frame. The first argument 'this'
    ///is assumed to be created with CreateThis.
    /// Arg3 is index in the function table.
    /// Note that we expect the variable-sized argument to be last.
    (78, CallDirect, r0: Reg8, p0: UInt8, p1: UInt16),
    /// Call a function with two args.
    /// Arg1 is the destination of the return value.
    /// Arg2 is the closure to invoke.
    /// Arg3 is the first argument.
    /// Arg4 is the second argument.
    (79, Call2, r0: Reg8, r1: Reg8, r2: Reg8, r3: Reg8),
    /// Call a function with three args.
    /// Arg1 is the destination of the return value.
    /// Arg2 is the closure to invoke.
    /// Arg3 is the first argument.
    /// Arg4 is the second argument.
    /// Arg5 is the third argument.
    (80, Call3, r0: Reg8, r1: Reg8, r2: Reg8, r3: Reg8, r4: Reg8),
    /// Call a function with four args.
    /// Arg1 is the destination of the return value.
    /// Arg2 is the closure to invoke.
    /// Arg3 is the first argument.
    /// Arg4 is the second argument.
    /// Arg5 is the third argument.
    /// Arg6 is the fourth argument.
    (81, Call4, r0: Reg8, r1: Reg8, r2: Reg8, r3: Reg8, r4: Reg8, r5: Reg8),
    /// Identical to Call, but allowing more arguments.
    (82, CallLong, r0: Reg8, r1: Reg8, p0: UInt32),
    /// Identical to Construct, but allowing more arguments.
    (83, ConstructLong, r0: Reg8, r1: Reg8, p0: UInt32),
    /// Identical to CallDirect, but the function index is 32-bit.
    (84, CallDirectLongIndex, r0: Reg8, p0: UInt8, p1: UInt32),
    /// Call a builtin function.
    /// Note this is NOT marked as a Ret target, because the callee is native
    /// and therefore never JS.
    /// Arg1 is the destination of the return value.
    /// Arg2 is the builtin number.
    /// Arg3 is the number of arguments, assumed to be found in reverse order
    ///from the end of the current frame.
    /// thisArg is set to "undefined".
    (85, CallBuiltin, r0: Reg8, p0: UInt8, p1: UInt8),
    /// Call a builtin function.
    /// Note this is NOT marked as a Ret target, because the callee is native
    /// and therefore never JS.
    /// Arg1 is the destination of the return value.
    /// Arg2 is the builtin number.
    /// Arg3 is the number of arguments, assumed to be found in reverse order
    ///from the end of the current frame.
    /// thisArg is set to "undefined".
    (86, CallBuiltinLong, r0: Reg8, p0: UInt8, p1: UInt32),
    /// Get a closure from a builtin function.
    /// Arg1 is the destination of the return value.
    /// Arg2 is the builtin number.
    (87, GetBuiltinClosure, r0: Reg8, p0: UInt8),
    /// Return a value from the current function.
    /// return Arg1;
    (88, Ret, r0: Reg8),
    /// Catch an exception (the first instruction in an exception handler).
    /// } catch(Arg1) {
    (89, Catch, r0: Reg8),
    /// ES6 18.2.1.1 PerformEval(Arg2, evalRealm, strictCaller=true, direct=true)
    /// Arg1 is the destination of the return value.
    /// Arg2 is the value to eval.
    (90, DirectEval, r0: Reg8, r1: Reg8),
    /// Throw an exception.
    /// throw Arg1;
    (91, Throw, r0: Reg8),
    /// If Arg2 is Empty, throw ReferenceError, otherwise move it into Arg1.
    /// Arg1 is the destination of the return value
    /// Arg2 is the value to check
    (92, ThrowIfEmpty, r0: Reg8, r1: Reg8),
    /// Implementation dependent debugger action.
    (93, Debugger, ),
    /// Fast check for an async interrupt request.
    (94, AsyncBreakCheck, ),
    /// Define a profile point.
    /// Arg1 is the function local profile point index. The first one will have the
    /// largest index. If there are more than 2^16 profile points in the function,
    /// all the overflowed profile points have index zero.
    (95, ProfilePoint, p0: UInt16),
    /// Create a closure.
    /// Arg1 is the register in which to store the closure.
    /// Arg2 is the current environment as loaded by GetEnvironment 0.
    /// Arg3 is index in the function table.
    (96, CreateClosure, r0: Reg8, r1: Reg8, p0: UInt16),
    /// Create a closure.
    /// Arg1 is the register in which to store the closure.
    /// Arg2 is the current environment as loaded by GetEnvironment 0.
    /// Arg3 is index in the function table.
    (97, CreateClosureLongIndex, r0: Reg8, r1: Reg8, p0: UInt32),
    /// Create a closure for a GeneratorFunction.
    /// Arg1 is the register in which to store the closure.
    /// Arg2 is the current environment as loaded by GetEnvironment 0.
    /// Arg3 is index in the function table.
    (98, CreateGeneratorClosure, r0: Reg8, r1: Reg8, p0: UInt16),
    /// Create a closure for a GeneratorFunction.
    /// Arg1 is the register in which to store the closure.
    /// Arg2 is the current environment as loaded by GetEnvironment 0.
    /// Arg3 is index in the function table.
    (99, CreateGeneratorClosureLongIndex, r0: Reg8, r1: Reg8, p0: UInt32),
    /// Create a closure for an AsyncFunction.
    /// Arg1 is the register in which to store the closure.
    /// Arg2 is the current environment as loaded by GetEnvironment 0.
    /// Arg3 is index in the function table.
    (100, CreateAsyncClosure, r0: Reg8, r1: Reg8, p0: UInt16),
    /// Create a closure for an AsyncFunction.
    /// Arg1 is the register in which to store the closure.
    /// Arg2 is the current environment as loaded by GetEnvironment 0.
    /// Arg3 is index in the function table.
    (101, CreateAsyncClosureLongIndex, r0: Reg8, r1: Reg8, p0: UInt32),
    /// Allocate an empty, uninitialized object (immediately before a constructor).
    /// Arg1 is the destination register.
    /// Arg2 is the prototype to assign it.
    /// Arg3 is the constructor closure that will be used*.
    /// * To allow internal constructors to have special objects allocated.
    (102, CreateThis, r0: Reg8, r1: Reg8, r2: Reg8),
    /// Choose the result of a constructor: 'this' or a returned object.
    /// Arg1 is the result.
    /// Arg2 is the 'this' object used for the constructor.
    /// Arg3 is the constructor's return value.
    /// Arg1 = Arg3 instanceof Object ? Arg3 : Arg2
    (103, SelectObject, r0: Reg8, r1: Reg8, r2: Reg8),
    /// Load a function parameter by index. Starts at 0 with 'this'.
    /// Arg1 = Arg2 == 0 ? this : arguments[Arg2 - 1];
    (104, LoadParam, r0: Reg8, p0: UInt8),
    /// Like LoadParam, but allows accessing arguments >= 255.
    (105, LoadParamLong, r0: Reg8, p0: UInt32),
    /// Load a constant integer value.
    (106, LoadConstUInt8, r0: Reg8, p0: UInt8),
    /// Load a constant integer value.
    (107, LoadConstInt, r0: Reg8, p0: Imm32),
    /// Load a constant double value.
    (108, LoadConstDouble, r0: Reg8, p0: Double),
    /// Load a constant string value by string table index.
    (109, LoadConstString, r0: Reg8, p0: StringIDUInt16),
    /// Load a constant string value by string table index.
    (110, LoadConstStringLongIndex, r0: Reg8, p0: StringIDUInt32),
    /// Load common constants.
    (111, LoadConstEmpty, r0: Reg8),
    /// Load common constants.
    (112, LoadConstUndefined, r0: Reg8),
    /// Load common constants.
    (113, LoadConstNull, r0: Reg8),
    /// Load common constants.
    (114, LoadConstTrue, r0: Reg8),
    /// Load common constants.
    (115, LoadConstFalse, r0: Reg8),
    /// Load common constants.
    (116, LoadConstZero, r0: Reg8),
    /// Coerce a value assumed to contain 'this' to an object using non-strict
    /// mode rules. Primitives are boxed, \c null or \c undefed produce the global
    /// object.
    /// Arg1 = coerce_to_object(Arg2)
    (117, CoerceThisNS, r0: Reg8, r1: Reg8),
    /// Obtain the raw \c this value and coerce it to an object. Equivalent to:
    /// \code
    /// LoadParamArg1, #0
    /// CoerceThisNS Arg1, Arg1
    /// \endcode
    (118, LoadThisNS, r0: Reg8),
    /// Convert a value to a number.
    /// Arg1 = Arg2 - 0
    /// Convert a value to a 32-bit signed integer.
    /// Arg1 = Arg2 | 0
    (119, ToNumber, r0: Reg8, r1: Reg8),
    /// Convert a value to a number.
    /// Arg1 = Arg2 - 0
    /// Convert a value to a 32-bit signed integer.
    /// Arg1 = Arg2 | 0
    (120, ToInt32, r0: Reg8, r1: Reg8),
    /// Convert a value to a string as if evaluating the expression:
    /// Arg1 = "" + Arg2
    /// In practice this means
    /// Arg1 = ToString(ToPrimitive(Arg2, PreferredType::NONE))
    /// with ToPrimitive (ES5.1 9.1) and ToString (ES5.1 9.8).
    (121, AddEmptyString, r0: Reg8, r1: Reg8),
    /// Get a property of the 'arguments' array by value.
    /// Arg1 is the result.
    /// Arg2 is the index.
    /// Arg3 is the lazy loaded register.
    /// Arg1 = arguments[Arg2]
    (122, GetArgumentsPropByVal, r0: Reg8, r1: Reg8, r2: Reg8),
    /// Get the length of the 'arguments' array.
    /// Arg1 is the result.
    /// Arg2 is the lazy loaded register.
    /// Arg1 = arguments.length
    (123, GetArgumentsLength, r0: Reg8, r1: Reg8),
    /// Create an actual 'arguments' array, if get-by-index and length isn't enough.
    /// Arg1 is the lazy loaded register, which afterwards will contain a proper
    ///object that can be used by non-*Arguments* opcodes like Return.
    (124, ReifyArguments, r0: Reg8),
    /// Create a regular expression.
    /// Arg1 is the result.
    /// Arg2 is the string index of the pattern.
    /// Arg3 is the string index of the flags.
    /// Arg4 is the regexp bytecode index in the regexp table.
    (125, CreateRegExp, r0: Reg8, p0: StringIDUInt32, p1: StringIDUInt32, p2: UInt32),
    /// Jump table switch - using a table of offset, jump to the offset of the given
    /// input or to the default block if out of range (or not right type)
    /// Arg 1 is the value to be branched upon
    /// Arg 2 is the relative offset of the jump table to be used by this
    /// instruction. Jump tables are appended to the bytecode. Arg 3 is the relative
    /// offset for the "default" jump. Arg 4 is the unsigned min value, if arg 1 is
    /// less than this value jmp to
    /// default block
    /// Arg 5 is the unsigned max value, if arg 1 is greater than this value jmp to
    /// default block.
    /// Given the above, the jump table entry for a given value (that is in range)
    /// is located at offset ip + arg2 + arg1 - arg4. We subtract arg4 to avoid
    /// wasting space when compiling denses switches that do not start at zero. Note
    /// that Arg2 is *unaligned* it is dynamically aligned at runtime.
    (126, SwitchImm, r0: Reg8, p0: UInt32, p1: Addr32, p2: UInt32, p3: UInt32),
    /// Start the generator by jumping to the next instruction to begin.
    /// Restore the stack frame if this generator has previously been suspended.
    (127, StartGenerator, ),
    /// Resume generator by performing one of the following user-requested actions:
    /// - next(val): Set Arg1 to val, Arg2 to false, run next instruction
    /// - return(val): Set Arg1 to val, Arg2 to true, run next instruction
    /// - throw(val): Throw val as an error
    /// Arg1 is the result provided by the user.
    /// Arg2 is a boolean which is true if the user requested a return().
    (128, ResumeGenerator, r0: Reg8, r1: Reg8),
    /// Set the generator status to complete, but do not return.
    (129, CompleteGenerator, ),
    /// Create a generator.
    /// Arg1 is the register in which to store the generator.
    /// Arg2 is the current environment as loaded by GetEnvironment 0.
    /// Arg3 is index in the function table.
    (130, CreateGenerator, r0: Reg8, r1: Reg8, p0: UInt16),
    /// Create a generator.
    /// Arg1 is the register in which to store the generator.
    /// Arg2 is the current environment as loaded by GetEnvironment 0.
    /// Arg3 is index in the function table.
    (131, CreateGeneratorLongIndex, r0: Reg8, r1: Reg8, p0: UInt32),
    /// Arg1 [out] is the result iterator or index.
    /// Arg2 [in/out] is the source. Output for either the source or next method.
    /// If source is an array with an unmodified [Symbol.iterator], the result is
    /// 0. Else the result is source[Symbol.iterator] and the output is the .next()
    /// method on the iterator.
    /// See IR.md for IteratorBeginInst.
    (132, IteratorBegin, r0: Reg8, r1: Reg8),
    /// Arg1 [out] is the result, or undefined if done.
    /// Arg2 [in/out] is the iterator or index.
    /// Arg2 [in] is the source or the next method.
    /// If iterator is undefined, result = undefined.
    /// If iterator is a number:
    /// If iterator is less than source.length, return source[iterator++]
    /// Else iterator = undefined and result = undefined
    /// Else:
    /// n = iterator.next()
    /// If n.done, iterator = undefined and result = undefined.
    /// Else result = n.value
    /// See IR.md for IteratorNextInst.
    (133, IteratorNext, r0: Reg8, r1: Reg8, r2: Reg8),
    /// Arg1 [in] is the iterator or array index.
    /// Arg2 is a bool indicating whether to ignore the inner exception.
    /// If the iterator is an object, call iterator.return().
    /// If Arg2 is true, ignore exceptions which are thrown by iterator.return().
    /// See IR.md for IteratorCloseInst.
    (134, IteratorClose, r0: Reg8, p0: UInt8),
    /// Unconditional branch to Arg1.
    /// Conditional branches to Arg1 based on Arg2.
    /// Jump if the value is undefined.
    /// Save the provided value, yield, and signal the VM to restart execution
    /// at the provided target.
    (135, Jmp, p0: Addr8),
    /// Unconditional branch to Arg1.
    /// Conditional branches to Arg1 based on Arg2.
    /// Jump if the value is undefined.
    /// Save the provided value, yield, and signal the VM to restart execution
    /// at the provided target.
    (136, JmpLong, p0: Addr32),
    /// Unconditional branch to Arg1.
    /// Conditional branches to Arg1 based on Arg2.
    /// Jump if the value is undefined.
    /// Save the provided value, yield, and signal the VM to restart execution
    /// at the provided target.
    (137, JmpTrue, p0: Addr8, r0: Reg8),
    /// Unconditional branch to Arg1.
    /// Conditional branches to Arg1 based on Arg2.
    /// Jump if the value is undefined.
    /// Save the provided value, yield, and signal the VM to restart execution
    /// at the provided target.
    (138, JmpTrueLong, p0: Addr32, r0: Reg8),
    /// Unconditional branch to Arg1.
    /// Conditional branches to Arg1 based on Arg2.
    /// Jump if the value is undefined.
    /// Save the provided value, yield, and signal the VM to restart execution
    /// at the provided target.
    (139, JmpFalse, p0: Addr8, r0: Reg8),
    /// Unconditional branch to Arg1.
    /// Conditional branches to Arg1 based on Arg2.
    /// Jump if the value is undefined.
    /// Save the provided value, yield, and signal the VM to restart execution
    /// at the provided target.
    (140, JmpFalseLong, p0: Addr32, r0: Reg8),
    /// Unconditional branch to Arg1.
    /// Conditional branches to Arg1 based on Arg2.
    /// Jump if the value is undefined.
    /// Save the provided value, yield, and signal the VM to restart execution
    /// at the provided target.
    (141, JmpUndefined, p0: Addr8, r0: Reg8),
    /// Unconditional branch to Arg1.
    /// Conditional branches to Arg1 based on Arg2.
    /// Jump if the value is undefined.
    /// Save the provided value, yield, and signal the VM to restart execution
    /// at the provided target.
    (142, JmpUndefinedLong, p0: Addr32, r0: Reg8),
    /// Unconditional branch to Arg1.
    /// Conditional branches to Arg1 based on Arg2.
    /// Jump if the value is undefined.
    /// Save the provided value, yield, and signal the VM to restart execution
    /// at the provided target.
    (143, SaveGenerator, p0: Addr8),
    /// Unconditional branch to Arg1.
    /// Conditional branches to Arg1 based on Arg2.
    /// Jump if the value is undefined.
    /// Save the provided value, yield, and signal the VM to restart execution
    /// at the provided target.
    (144, SaveGeneratorLong, p0: Addr32),
    /// Conditional branches to Arg1 based on Arg2 and Arg3.
    /// The *N branches assume numbers and are illegal for other types.
    /// Not conditionals are required for NaN comparisons
    /// Since we want to be able to reorder targets to allow for fall-throughs,
    /// we need to be able to say "jump when not less than to BB2" instead of
    /// "jump when less than to BB1".
    /// Since NaN comparisons always return false, "not less" != "greater or equal"
    (145, JLess, p0: Addr8, r0: Reg8, r1: Reg8),
    /// Conditional branches to Arg1 based on Arg2 and Arg3.
    /// The *N branches assume numbers and are illegal for other types.
    /// Not conditionals are required for NaN comparisons
    /// Since we want to be able to reorder targets to allow for fall-throughs,
    /// we need to be able to say "jump when not less than to BB2" instead of
    /// "jump when less than to BB1".
    /// Since NaN comparisons always return false, "not less" != "greater or equal"
    (146, JLessLong, p0: Addr32, r0: Reg8, r1: Reg8),
    /// Conditional branches to Arg1 based on Arg2 and Arg3.
    /// The *N branches assume numbers and are illegal for other types.
    /// Not conditionals are required for NaN comparisons
    /// Since we want to be able to reorder targets to allow for fall-throughs,
    /// we need to be able to say "jump when not less than to BB2" instead of
    /// "jump when less than to BB1".
    /// Since NaN comparisons always return false, "not less" != "greater or equal"
    (147, JNotLess, p0: Addr8, r0: Reg8, r1: Reg8),
    /// Conditional branches to Arg1 based on Arg2 and Arg3.
    /// The *N branches assume numbers and are illegal for other types.
    /// Not conditionals are required for NaN comparisons
    /// Since we want to be able to reorder targets to allow for fall-throughs,
    /// we need to be able to say "jump when not less than to BB2" instead of
    /// "jump when less than to BB1".
    /// Since NaN comparisons always return false, "not less" != "greater or equal"
    (148, JNotLessLong, p0: Addr32, r0: Reg8, r1: Reg8),
    /// Conditional branches to Arg1 based on Arg2 and Arg3.
    /// The *N branches assume numbers and are illegal for other types.
    /// Not conditionals are required for NaN comparisons
    /// Since we want to be able to reorder targets to allow for fall-throughs,
    /// we need to be able to say "jump when not less than to BB2" instead of
    /// "jump when less than to BB1".
    /// Since NaN comparisons always return false, "not less" != "greater or equal"
    (149, JLessN, p0: Addr8, r0: Reg8, r1: Reg8),
    /// Conditional branches to Arg1 based on Arg2 and Arg3.
    /// The *N branches assume numbers and are illegal for other types.
    /// Not conditionals are required for NaN comparisons
    /// Since we want to be able to reorder targets to allow for fall-throughs,
    /// we need to be able to say "jump when not less than to BB2" instead of
    /// "jump when less than to BB1".
    /// Since NaN comparisons always return false, "not less" != "greater or equal"
    (150, JLessNLong, p0: Addr32, r0: Reg8, r1: Reg8),
    /// Conditional branches to Arg1 based on Arg2 and Arg3.
    /// The *N branches assume numbers and are illegal for other types.
    /// Not conditionals are required for NaN comparisons
    /// Since we want to be able to reorder targets to allow for fall-throughs,
    /// we need to be able to say "jump when not less than to BB2" instead of
    /// "jump when less than to BB1".
    /// Since NaN comparisons always return false, "not less" != "greater or equal"
    (151, JNotLessN, p0: Addr8, r0: Reg8, r1: Reg8),
    /// Conditional branches to Arg1 based on Arg2 and Arg3.
    /// The *N branches assume numbers and are illegal for other types.
    /// Not conditionals are required for NaN comparisons
    /// Since we want to be able to reorder targets to allow for fall-throughs,
    /// we need to be able to say "jump when not less than to BB2" instead of
    /// "jump when less than to BB1".
    /// Since NaN comparisons always return false, "not less" != "greater or equal"
    (152, JNotLessNLong, p0: Addr32, r0: Reg8, r1: Reg8),
    /// Conditional branches to Arg1 based on Arg2 and Arg3.
    /// The *N branches assume numbers and are illegal for other types.
    /// Not conditionals are required for NaN comparisons
    /// Since we want to be able to reorder targets to allow for fall-throughs,
    /// we need to be able to say "jump when not less than to BB2" instead of
    /// "jump when less than to BB1".
    /// Since NaN comparisons always return false, "not less" != "greater or equal"
    (153, JLessEqual, p0: Addr8, r0: Reg8, r1: Reg8),
    /// Conditional branches to Arg1 based on Arg2 and Arg3.
    /// The *N branches assume numbers and are illegal for other types.
    /// Not conditionals are required for NaN comparisons
    /// Since we want to be able to reorder targets to allow for fall-throughs,
    /// we need to be able to say "jump when not less than to BB2" instead of
    /// "jump when less than to BB1".
    /// Since NaN comparisons always return false, "not less" != "greater or equal"
    (154, JLessEqualLong, p0: Addr32, r0: Reg8, r1: Reg8),
    /// Conditional branches to Arg1 based on Arg2 and Arg3.
    /// The *N branches assume numbers and are illegal for other types.
    /// Not conditionals are required for NaN comparisons
    /// Since we want to be able to reorder targets to allow for fall-throughs,
    /// we need to be able to say "jump when not less than to BB2" instead of
    /// "jump when less than to BB1".
    /// Since NaN comparisons always return false, "not less" != "greater or equal"
    (155, JNotLessEqual, p0: Addr8, r0: Reg8, r1: Reg8),
    /// Conditional branches to Arg1 based on Arg2 and Arg3.
    /// The *N branches assume numbers and are illegal for other types.
    /// Not conditionals are required for NaN comparisons
    /// Since we want to be able to reorder targets to allow for fall-throughs,
    /// we need to be able to say "jump when not less than to BB2" instead of
    /// "jump when less than to BB1".
    /// Since NaN comparisons always return false, "not less" != "greater or equal"
    (156, JNotLessEqualLong, p0: Addr32, r0: Reg8, r1: Reg8),
    /// Conditional branches to Arg1 based on Arg2 and Arg3.
    /// The *N branches assume numbers and are illegal for other types.
    /// Not conditionals are required for NaN comparisons
    /// Since we want to be able to reorder targets to allow for fall-throughs,
    /// we need to be able to say "jump when not less than to BB2" instead of
    /// "jump when less than to BB1".
    /// Since NaN comparisons always return false, "not less" != "greater or equal"
    (157, JLessEqualN, p0: Addr8, r0: Reg8, r1: Reg8),
    /// Conditional branches to Arg1 based on Arg2 and Arg3.
    /// The *N branches assume numbers and are illegal for other types.
    /// Not conditionals are required for NaN comparisons
    /// Since we want to be able to reorder targets to allow for fall-throughs,
    /// we need to be able to say "jump when not less than to BB2" instead of
    /// "jump when less than to BB1".
    /// Since NaN comparisons always return false, "not less" != "greater or equal"
    (158, JLessEqualNLong, p0: Addr32, r0: Reg8, r1: Reg8),
    /// Conditional branches to Arg1 based on Arg2 and Arg3.
    /// The *N branches assume numbers and are illegal for other types.
    /// Not conditionals are required for NaN comparisons
    /// Since we want to be able to reorder targets to allow for fall-throughs,
    /// we need to be able to say "jump when not less than to BB2" instead of
    /// "jump when less than to BB1".
    /// Since NaN comparisons always return false, "not less" != "greater or equal"
    (159, JNotLessEqualN, p0: Addr8, r0: Reg8, r1: Reg8),
    /// Conditional branches to Arg1 based on Arg2 and Arg3.
    /// The *N branches assume numbers and are illegal for other types.
    /// Not conditionals are required for NaN comparisons
    /// Since we want to be able to reorder targets to allow for fall-throughs,
    /// we need to be able to say "jump when not less than to BB2" instead of
    /// "jump when less than to BB1".
    /// Since NaN comparisons always return false, "not less" != "greater or equal"
    (160, JNotLessEqualNLong, p0: Addr32, r0: Reg8, r1: Reg8),
    /// Conditional branches to Arg1 based on Arg2 and Arg3.
    /// The *N branches assume numbers and are illegal for other types.
    /// Not conditionals are required for NaN comparisons
    /// Since we want to be able to reorder targets to allow for fall-throughs,
    /// we need to be able to say "jump when not less than to BB2" instead of
    /// "jump when less than to BB1".
    /// Since NaN comparisons always return false, "not less" != "greater or equal"
    (161, JGreater, p0: Addr8, r0: Reg8, r1: Reg8),
    /// Conditional branches to Arg1 based on Arg2 and Arg3.
    /// The *N branches assume numbers and are illegal for other types.
    /// Not conditionals are required for NaN comparisons
    /// Since we want to be able to reorder targets to allow for fall-throughs,
    /// we need to be able to say "jump when not less than to BB2" instead of
    /// "jump when less than to BB1".
    /// Since NaN comparisons always return false, "not less" != "greater or equal"
    (162, JGreaterLong, p0: Addr32, r0: Reg8, r1: Reg8),
    /// Conditional branches to Arg1 based on Arg2 and Arg3.
    /// The *N branches assume numbers and are illegal for other types.
    /// Not conditionals are required for NaN comparisons
    /// Since we want to be able to reorder targets to allow for fall-throughs,
    /// we need to be able to say "jump when not less than to BB2" instead of
    /// "jump when less than to BB1".
    /// Since NaN comparisons always return false, "not less" != "greater or equal"
    (163, JNotGreater, p0: Addr8, r0: Reg8, r1: Reg8),
    /// Conditional branches to Arg1 based on Arg2 and Arg3.
    /// The *N branches assume numbers and are illegal for other types.
    /// Not conditionals are required for NaN comparisons
    /// Since we want to be able to reorder targets to allow for fall-throughs,
    /// we need to be able to say "jump when not less than to BB2" instead of
    /// "jump when less than to BB1".
    /// Since NaN comparisons always return false, "not less" != "greater or equal"
    (164, JNotGreaterLong, p0: Addr32, r0: Reg8, r1: Reg8),
    /// Conditional branches to Arg1 based on Arg2 and Arg3.
    /// The *N branches assume numbers and are illegal for other types.
    /// Not conditionals are required for NaN comparisons
    /// Since we want to be able to reorder targets to allow for fall-throughs,
    /// we need to be able to say "jump when not less than to BB2" instead of
    /// "jump when less than to BB1".
    /// Since NaN comparisons always return false, "not less" != "greater or equal"
    (165, JGreaterN, p0: Addr8, r0: Reg8, r1: Reg8),
    /// Conditional branches to Arg1 based on Arg2 and Arg3.
    /// The *N branches assume numbers and are illegal for other types.
    /// Not conditionals are required for NaN comparisons
    /// Since we want to be able to reorder targets to allow for fall-throughs,
    /// we need to be able to say "jump when not less than to BB2" instead of
    /// "jump when less than to BB1".
    /// Since NaN comparisons always return false, "not less" != "greater or equal"
    (166, JGreaterNLong, p0: Addr32, r0: Reg8, r1: Reg8),
    /// Conditional branches to Arg1 based on Arg2 and Arg3.
    /// The *N branches assume numbers and are illegal for other types.
    /// Not conditionals are required for NaN comparisons
    /// Since we want to be able to reorder targets to allow for fall-throughs,
    /// we need to be able to say "jump when not less than to BB2" instead of
    /// "jump when less than to BB1".
    /// Since NaN comparisons always return false, "not less" != "greater or equal"
    (167, JNotGreaterN, p0: Addr8, r0: Reg8, r1: Reg8),
    /// Conditional branches to Arg1 based on Arg2 and Arg3.
    /// The *N branches assume numbers and are illegal for other types.
    /// Not conditionals are required for NaN comparisons
    /// Since we want to be able to reorder targets to allow for fall-throughs,
    /// we need to be able to say "jump when not less than to BB2" instead of
    /// "jump when less than to BB1".
    /// Since NaN comparisons always return false, "not less" != "greater or equal"
    (168, JNotGreaterNLong, p0: Addr32, r0: Reg8, r1: Reg8),
    /// Conditional branches to Arg1 based on Arg2 and Arg3.
    /// The *N branches assume numbers and are illegal for other types.
    /// Not conditionals are required for NaN comparisons
    /// Since we want to be able to reorder targets to allow for fall-throughs,
    /// we need to be able to say "jump when not less than to BB2" instead of
    /// "jump when less than to BB1".
    /// Since NaN comparisons always return false, "not less" != "greater or equal"
    (169, JGreaterEqual, p0: Addr8, r0: Reg8, r1: Reg8),
    /// Conditional branches to Arg1 based on Arg2 and Arg3.
    /// The *N branches assume numbers and are illegal for other types.
    /// Not conditionals are required for NaN comparisons
    /// Since we want to be able to reorder targets to allow for fall-throughs,
    /// we need to be able to say "jump when not less than to BB2" instead of
    /// "jump when less than to BB1".
    /// Since NaN comparisons always return false, "not less" != "greater or equal"
    (170, JGreaterEqualLong, p0: Addr32, r0: Reg8, r1: Reg8),
    /// Conditional branches to Arg1 based on Arg2 and Arg3.
    /// The *N branches assume numbers and are illegal for other types.
    /// Not conditionals are required for NaN comparisons
    /// Since we want to be able to reorder targets to allow for fall-throughs,
    /// we need to be able to say "jump when not less than to BB2" instead of
    /// "jump when less than to BB1".
    /// Since NaN comparisons always return false, "not less" != "greater or equal"
    (171, JNotGreaterEqual, p0: Addr8, r0: Reg8, r1: Reg8),
    /// Conditional branches to Arg1 based on Arg2 and Arg3.
    /// The *N branches assume numbers and are illegal for other types.
    /// Not conditionals are required for NaN comparisons
    /// Since we want to be able to reorder targets to allow for fall-throughs,
    /// we need to be able to say "jump when not less than to BB2" instead of
    /// "jump when less than to BB1".
    /// Since NaN comparisons always return false, "not less" != "greater or equal"
    (172, JNotGreaterEqualLong, p0: Addr32, r0: Reg8, r1: Reg8),
    /// Conditional branches to Arg1 based on Arg2 and Arg3.
    /// The *N branches assume numbers and are illegal for other types.
    /// Not conditionals are required for NaN comparisons
    /// Since we want to be able to reorder targets to allow for fall-throughs,
    /// we need to be able to say "jump when not less than to BB2" instead of
    /// "jump when less than to BB1".
    /// Since NaN comparisons always return false, "not less" != "greater or equal"
    (173, JGreaterEqualN, p0: Addr8, r0: Reg8, r1: Reg8),
    /// Conditional branches to Arg1 based on Arg2 and Arg3.
    /// The *N branches assume numbers and are illegal for other types.
    /// Not conditionals are required for NaN comparisons
    /// Since we want to be able to reorder targets to allow for fall-throughs,
    /// we need to be able to say "jump when not less than to BB2" instead of
    /// "jump when less than to BB1".
    /// Since NaN comparisons always return false, "not less" != "greater or equal"
    (174, JGreaterEqualNLong, p0: Addr32, r0: Reg8, r1: Reg8),
    /// Conditional branches to Arg1 based on Arg2 and Arg3.
    /// The *N branches assume numbers and are illegal for other types.
    /// Not conditionals are required for NaN comparisons
    /// Since we want to be able to reorder targets to allow for fall-throughs,
    /// we need to be able to say "jump when not less than to BB2" instead of
    /// "jump when less than to BB1".
    /// Since NaN comparisons always return false, "not less" != "greater or equal"
    (175, JNotGreaterEqualN, p0: Addr8, r0: Reg8, r1: Reg8),
    /// Conditional branches to Arg1 based on Arg2 and Arg3.
    /// The *N branches assume numbers and are illegal for other types.
    /// Not conditionals are required for NaN comparisons
    /// Since we want to be able to reorder targets to allow for fall-throughs,
    /// we need to be able to say "jump when not less than to BB2" instead of
    /// "jump when less than to BB1".
    /// Since NaN comparisons always return false, "not less" != "greater or equal"
    (176, JNotGreaterEqualNLong, p0: Addr32, r0: Reg8, r1: Reg8),
    /// Conditional branches to Arg1 based on Arg2 and Arg3.
    /// The *N branches assume numbers and are illegal for other types.
    /// Not conditionals are required for NaN comparisons
    /// Since we want to be able to reorder targets to allow for fall-throughs,
    /// we need to be able to say "jump when not less than to BB2" instead of
    /// "jump when less than to BB1".
    /// Since NaN comparisons always return false, "not less" != "greater or equal"
    (177, JEqual, p0: Addr8, r0: Reg8, r1: Reg8),
    /// Conditional branches to Arg1 based on Arg2 and Arg3.
    /// The *N branches assume numbers and are illegal for other types.
    /// Not conditionals are required for NaN comparisons
    /// Since we want to be able to reorder targets to allow for fall-throughs,
    /// we need to be able to say "jump when not less than to BB2" instead of
    /// "jump when less than to BB1".
    /// Since NaN comparisons always return false, "not less" != "greater or equal"
    (178, JEqualLong, p0: Addr32, r0: Reg8, r1: Reg8),
    /// Conditional branches to Arg1 based on Arg2 and Arg3.
    /// The *N branches assume numbers and are illegal for other types.
    /// Not conditionals are required for NaN comparisons
    /// Since we want to be able to reorder targets to allow for fall-throughs,
    /// we need to be able to say "jump when not less than to BB2" instead of
    /// "jump when less than to BB1".
    /// Since NaN comparisons always return false, "not less" != "greater or equal"
    (179, JNotEqual, p0: Addr8, r0: Reg8, r1: Reg8),
    /// Conditional branches to Arg1 based on Arg2 and Arg3.
    /// The *N branches assume numbers and are illegal for other types.
    /// Not conditionals are required for NaN comparisons
    /// Since we want to be able to reorder targets to allow for fall-throughs,
    /// we need to be able to say "jump when not less than to BB2" instead of
    /// "jump when less than to BB1".
    /// Since NaN comparisons always return false, "not less" != "greater or equal"
    (180, JNotEqualLong, p0: Addr32, r0: Reg8, r1: Reg8),
    /// Conditional branches to Arg1 based on Arg2 and Arg3.
    /// The *N branches assume numbers and are illegal for other types.
    /// Not conditionals are required for NaN comparisons
    /// Since we want to be able to reorder targets to allow for fall-throughs,
    /// we need to be able to say "jump when not less than to BB2" instead of
    /// "jump when less than to BB1".
    /// Since NaN comparisons always return false, "not less" != "greater or equal"
    (181, JStrictEqual, p0: Addr8, r0: Reg8, r1: Reg8),
    /// Conditional branches to Arg1 based on Arg2 and Arg3.
    /// The *N branches assume numbers and are illegal for other types.
    /// Not conditionals are required for NaN comparisons
    /// Since we want to be able to reorder targets to allow for fall-throughs,
    /// we need to be able to say "jump when not less than to BB2" instead of
    /// "jump when less than to BB1".
    /// Since NaN comparisons always return false, "not less" != "greater or equal"
    (182, JStrictEqualLong, p0: Addr32, r0: Reg8, r1: Reg8),
    /// Conditional branches to Arg1 based on Arg2 and Arg3.
    /// The *N branches assume numbers and are illegal for other types.
    /// Not conditionals are required for NaN comparisons
    /// Since we want to be able to reorder targets to allow for fall-throughs,
    /// we need to be able to say "jump when not less than to BB2" instead of
    /// "jump when less than to BB1".
    /// Since NaN comparisons always return false, "not less" != "greater or equal"
    (183, JStrictNotEqual, p0: Addr8, r0: Reg8, r1: Reg8),
    /// Conditional branches to Arg1 based on Arg2 and Arg3.
    /// The *N branches assume numbers and are illegal for other types.
    /// Not conditionals are required for NaN comparisons
    /// Since we want to be able to reorder targets to allow for fall-throughs,
    /// we need to be able to say "jump when not less than to BB2" instead of
    /// "jump when less than to BB1".
    /// Since NaN comparisons always return false, "not less" != "greater or equal"
    (184, JStrictNotEqualLong, p0: Addr32, r0: Reg8, r1: Reg8),
    /// Arg1 = Arg2 + Arg3 (32-bit integer addition)
    /// Arg1 = Arg2 - Arg3 (32-bit integer subtraction)
    /// Arg1 = Arg2 * Arg3 (32-bit integer multiplication)
    /// Arg1 = Arg2 / Arg3 (32-bit signed integer division)
    /// Arg1 = Arg2 / Arg3 (32-bit unsigned integer division)
    (185, Add32, r0: Reg8, r1: Reg8, r2: Reg8),
    /// Arg1 = Arg2 + Arg3 (32-bit integer addition)
    /// Arg1 = Arg2 - Arg3 (32-bit integer subtraction)
    /// Arg1 = Arg2 * Arg3 (32-bit integer multiplication)
    /// Arg1 = Arg2 / Arg3 (32-bit signed integer division)
    /// Arg1 = Arg2 / Arg3 (32-bit unsigned integer division)
    (186, Sub32, r0: Reg8, r1: Reg8, r2: Reg8),
    /// Arg1 = Arg2 + Arg3 (32-bit integer addition)
    /// Arg1 = Arg2 - Arg3 (32-bit integer subtraction)
    /// Arg1 = Arg2 * Arg3 (32-bit integer multiplication)
    /// Arg1 = Arg2 / Arg3 (32-bit signed integer division)
    /// Arg1 = Arg2 / Arg3 (32-bit unsigned integer division)
    (187, Mul32, r0: Reg8, r1: Reg8, r2: Reg8),
    /// Arg1 = Arg2 + Arg3 (32-bit integer addition)
    /// Arg1 = Arg2 - Arg3 (32-bit integer subtraction)
    /// Arg1 = Arg2 * Arg3 (32-bit integer multiplication)
    /// Arg1 = Arg2 / Arg3 (32-bit signed integer division)
    /// Arg1 = Arg2 / Arg3 (32-bit unsigned integer division)
    (188, Divi32, r0: Reg8, r1: Reg8, r2: Reg8),
    /// Arg1 = Arg2 + Arg3 (32-bit integer addition)
    /// Arg1 = Arg2 - Arg3 (32-bit integer subtraction)
    /// Arg1 = Arg2 * Arg3 (32-bit integer multiplication)
    /// Arg1 = Arg2 / Arg3 (32-bit signed integer division)
    /// Arg1 = Arg2 / Arg3 (32-bit unsigned integer division)
    (189, Divu32, r0: Reg8, r1: Reg8, r2: Reg8),
    /// Arg1 = HEAP8[Arg3] (load signed 8-bit integer)
    /// Arg1 = HEAPU8[Arg3] (load unsigned 8-bit integer)
    /// Arg1 = HEAP16[Arg3 >> 1] (load signed 16-bit integer)
    /// Arg1 = HEAPU16[Arg3 >> 1] (load unsigned 16-bit integer)
    /// Arg1 = HEAP32[Arg3 >> 2] (load signed 32-bit integer)
    /// Arg1 = HEAPU32[Arg3 >> 2] (load unsigned 32-bit integer)
    (190, Loadi8, r0: Reg8, r1: Reg8, r2: Reg8),
    /// Arg1 = HEAP8[Arg3] (load signed 8-bit integer)
    /// Arg1 = HEAPU8[Arg3] (load unsigned 8-bit integer)
    /// Arg1 = HEAP16[Arg3 >> 1] (load signed 16-bit integer)
    /// Arg1 = HEAPU16[Arg3 >> 1] (load unsigned 16-bit integer)
    /// Arg1 = HEAP32[Arg3 >> 2] (load signed 32-bit integer)
    /// Arg1 = HEAPU32[Arg3 >> 2] (load unsigned 32-bit integer)
    (191, Loadu8, r0: Reg8, r1: Reg8, r2: Reg8),
    /// Arg1 = HEAP8[Arg3] (load signed 8-bit integer)
    /// Arg1 = HEAPU8[Arg3] (load unsigned 8-bit integer)
    /// Arg1 = HEAP16[Arg3 >> 1] (load signed 16-bit integer)
    /// Arg1 = HEAPU16[Arg3 >> 1] (load unsigned 16-bit integer)
    /// Arg1 = HEAP32[Arg3 >> 2] (load signed 32-bit integer)
    /// Arg1 = HEAPU32[Arg3 >> 2] (load unsigned 32-bit integer)
    (192, Loadi16, r0: Reg8, r1: Reg8, r2: Reg8),
    /// Arg1 = HEAP8[Arg3] (load signed 8-bit integer)
    /// Arg1 = HEAPU8[Arg3] (load unsigned 8-bit integer)
    /// Arg1 = HEAP16[Arg3 >> 1] (load signed 16-bit integer)
    /// Arg1 = HEAPU16[Arg3 >> 1] (load unsigned 16-bit integer)
    /// Arg1 = HEAP32[Arg3 >> 2] (load signed 32-bit integer)
    /// Arg1 = HEAPU32[Arg3 >> 2] (load unsigned 32-bit integer)
    (193, Loadu16, r0: Reg8, r1: Reg8, r2: Reg8),
    /// Arg1 = HEAP8[Arg3] (load signed 8-bit integer)
    /// Arg1 = HEAPU8[Arg3] (load unsigned 8-bit integer)
    /// Arg1 = HEAP16[Arg3 >> 1] (load signed 16-bit integer)
    /// Arg1 = HEAPU16[Arg3 >> 1] (load unsigned 16-bit integer)
    /// Arg1 = HEAP32[Arg3 >> 2] (load signed 32-bit integer)
    /// Arg1 = HEAPU32[Arg3 >> 2] (load unsigned 32-bit integer)
    (194, Loadi32, r0: Reg8, r1: Reg8, r2: Reg8),
    /// Arg1 = HEAP8[Arg3] (load signed 8-bit integer)
    /// Arg1 = HEAPU8[Arg3] (load unsigned 8-bit integer)
    /// Arg1 = HEAP16[Arg3 >> 1] (load signed 16-bit integer)
    /// Arg1 = HEAPU16[Arg3 >> 1] (load unsigned 16-bit integer)
    /// Arg1 = HEAP32[Arg3 >> 2] (load signed 32-bit integer)
    /// Arg1 = HEAPU32[Arg3 >> 2] (load unsigned 32-bit integer)
    (195, Loadu32, r0: Reg8, r1: Reg8, r2: Reg8),
    /// HEAP8[Arg2] = Arg3 (store signed or unsigned 8-bit integer)
    /// HEAP16[Arg2] = Arg3 (store signed or unsigned 16-bit integer)
    /// HEAP32[Arg2] = Arg3 (store signed or unsigned 32-bit integer)
    (196, Store8, r0: Reg8, r1: Reg8, r2: Reg8),
    /// HEAP8[Arg2] = Arg3 (store signed or unsigned 8-bit integer)
    /// HEAP16[Arg2] = Arg3 (store signed or unsigned 16-bit integer)
    /// HEAP32[Arg2] = Arg3 (store signed or unsigned 32-bit integer)
    (197, Store16, r0: Reg8, r1: Reg8, r2: Reg8),
    /// HEAP8[Arg2] = Arg3 (store signed or unsigned 8-bit integer)
    /// HEAP16[Arg2] = Arg3 (store signed or unsigned 16-bit integer)
    /// HEAP32[Arg2] = Arg3 (store signed or unsigned 32-bit integer)
    (198, Store32, r0: Reg8, r1: Reg8, r2: Reg8)
);
