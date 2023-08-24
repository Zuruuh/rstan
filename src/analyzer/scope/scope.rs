use crate::{
    r#type::Type,
    reflection::{ClassMemberAccessAnswerer, NamespaceAnswerer},
};
/**

    fn getFile(): string;

    fn getFileDescription(): string;

    fn isDeclareStrictTypes(): bool;

    /**
     * @phpstan-assert-if-true !null $this->getTraitReflection()
     */
    fn isInTrait(): bool;

    fn getTraitReflection(): ?ClassReflection;

    /**
     * @return FunctionReflection|ExtendedMethodReflection|null
     */
    fn getFunction();

    fn getFunctionName(): ?string;

    fn getParentScope(): ?self;

    fn hasVariableType(string $variableName): TrinaryLogic;

    fn getVariableType(string $variableName): Type;

    fn canAnyVariableExist(): bool;

    /**
     * @return array<int, string>
     */
    fn getDefinedVariables(): array;

    fn hasConstant(Name $name): bool;

    fn getPropertyReflection(Type $typeWithProperty, string $propertyName): ?PropertyReflection;

    fn getMethodReflection(Type $typeWithMethod, string $methodName): ?ExtendedMethodReflection;

    fn getConstantReflection(Type $typeWithConstant, string $constantName): ?ConstantReflection;

    fn getIterableKeyType(Type $iteratee): Type;

    fn getIterableValueType(Type $iteratee): Type;

    fn isInAnonymousFunction(): bool;

    fn getAnonymousFunctionReflection(): ?ParametersAcceptor;

    fn getAnonymousFunctionReturnType(): ?Type;

    fn getType(Expr $node): Type;

    fn getNativeType(Expr $expr): Type;

    /**
     * @deprecated Use getNativeType()
     */
    fn doNotTreatPhpDocTypesAsCertain(): self;

    fn resolveName(Name $name): string;

    fn resolveTypeByName(Name $name): TypeWithClassName;

    /**
     * @param mixed $value
     */
    fn getTypeFromValue($value): Type;

    /** @deprecated use hasExpressionType instead */
    fn isSpecified(Expr $node): bool;

    fn hasExpressionType(Expr $node): TrinaryLogic;

    fn isInClassExists(string $className): bool;

    fn isInFunctionExists(string $functionName): bool;

    fn isInClosureBind(): bool;

    fn isParameterValueNullable(Param $parameter): bool;

    /**
     * @param Node\Name|Node\Identifier|Node\ComplexType|null $type
     */
*/

pub trait Scope: NamespaceAnswerer + ClassMemberAccessAnswerer {
    fn get_function_type(&self, r#type: String, is_nullable: bool, is_variadic: bool) -> dyn Type;
    fn is_in_expression_assign(&self, expr: Expr) -> bool;
    fn is_undefined_expression_allowed(&self, expr: Expr) -> bool;
    fn filter_by_truthy_value(&self, expr: Expr) -> Self;
    fn filter_by_falsey_value(&self, expr: Expr) -> Self;
    fn is_in_first_level_statement(&self) -> bool;
}
