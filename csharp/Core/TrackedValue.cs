namespace AllColors;

public sealed class TrackedValue<T> :
    IEquatable<TrackedValue<T>>, IEquatable<T>,
    IFormattable
{
    internal int ParentIndex { get; set; }

    public TrackedListSet<T> Parent { get; }

    public T Value { get; }

    internal TrackedValue(TrackedListSet<T> parent, T value)
    {
        this.ParentIndex = -1;
        this.Parent = parent;
        this.Value = value;
    }

    public bool IsInParent()
    {
        return ParentIndex > -1;
    }

    public bool TryAddToParent()
    {
        return Parent.TryAdd(this);
    }
    
    public bool Equals(TrackedValue<T>? trackedValue)
    {
        return trackedValue is not null &&
               ReferenceEquals(this.Parent, trackedValue.Parent) &&
               EqualityComparer<T>.Default.Equals(this.Value, trackedValue.Value);
    }
    
    public bool Equals(T? value)
    {
        return EqualityComparer<T>.Default.Equals(this.Value, value);
    }

    public override bool Equals(object? obj)
    {
        if (obj is TrackedValue<T> trackedValue)
            return Equals(trackedValue);
        if (obj is T value)
            return Equals(value);
        return false;
    }
    public override int GetHashCode()
    {
        return HashCode.Combine(Parent, Value);
    }

    public string ToString(string? format, IFormatProvider? formatProvider = default)
    {
        if (Value is IFormattable)
        {
            return ((IFormattable)Value).ToString(format, formatProvider);
        }
        return ToString();
    }

    public override string ToString()
    {
        return Value?.ToString() ?? "";
    }
}