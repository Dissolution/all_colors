using System.Diagnostics.CodeAnalysis;

namespace AllColors;

public static class Option
{
    public static Option<T> Some<T>([NotNull] T value)
    {
        ArgumentNullException.ThrowIfNull(value);
        return new Option<T>(true, value);
    }

    public static Option<T> None<T>()
    {
        return new Option<T>(false, default);
    }
}

public readonly struct Option<T> : IEquatable<T>, IEnumerable<T>
{
    public static implicit operator Option<T>([NotNull] T value) => Some(value);
    
    public static bool operator ==(Option<T> left, Option<T> right) => left.Equals(right);
    public static bool operator !=(Option<T> left, Option<T> right) => !left.Equals(right);

    public static Option<T> Some([NotNull] T value)
    {
        ArgumentNullException.ThrowIfNull(value);
        return new Option<T>(true, value);
    }

    public static Option<T> None()
    {
        return new Option<T>(false, default);
    }
    
    
    private readonly bool _hasValue;
    private readonly T? _value;

    internal Option(bool hasValue, T? value)
    {
        _hasValue = hasValue;
        _value = value;
    }

    public bool IsNone() => !_hasValue;
    public bool IsSome() => _hasValue;
    public bool IsSome([NotNullWhen(true)] out T? value)
    {
        value = _value;
        return _hasValue;
    }

    public bool Equals(Option<T> option)
    {
        if (!_hasValue)
            return !option._hasValue;
        return option._hasValue &&
               EqualityComparer<T>.Default.Equals(_value, option._value);
    }

    public bool Equals(T? value)
    {
        return _hasValue && EqualityComparer<T>.Default.Equals(_value, value);
    }
    IEnumerator IEnumerable.GetEnumerator() => GetEnumerator();
    public IEnumerator<T> GetEnumerator()
    {
        if (_hasValue)
            yield return _value!;
    }

    public override bool Equals(object? obj)
    {
        if (obj is Option<T> option) return Equals(option);
        if (obj is T value) return Equals(value);
        return false;
    }
    public override int GetHashCode()
    {
        if (!_hasValue) return 0;
        return _value!.GetHashCode();
    }
    public override string ToString()
    {
        if (!_hasValue)
            return "None";
        return $"Some<{typeof(T).Name}>({_value})";
    }
}