using System.Numerics;

namespace AllColors.Originals.IMPL2;

public sealed class KeyTuple<TKey, T1> : KeyTuple<TKey>
    where TKey : IEquatable<TKey>, IComparable<TKey>
{
    public T1 Item1 { get; }
    
    public KeyTuple(TKey key, T1 item1) : base(key)
    {
        this.Item1 = item1;
    }

    public override string ToString()
    {
        return $"({Key}:{Item1})";
    }
}


public class KeyTuple<TKey> : 
    IEquatable<TKey>, 
    IEquatable<KeyTuple<TKey>>,
    IEqualityOperators<KeyTuple<TKey>, KeyTuple<TKey>, bool>,
    IEqualityOperators<KeyTuple<TKey>, TKey, bool>,
    IComparisonOperators<KeyTuple<TKey>, KeyTuple<TKey>, bool>,
    IComparisonOperators<KeyTuple<TKey>, TKey, bool>,
    IComparable<TKey>, 
    IComparable<KeyTuple<TKey>>
    where TKey : IEquatable<TKey>, IComparable<TKey>
{
    public static bool operator ==(KeyTuple<TKey>? left, KeyTuple<TKey>? right)
    {
        if (ReferenceEquals(left, right)) return true;
        if (left is null || right is null) return false;
        return EqualityComparer<TKey>.Default.Equals(left.Key, right.Key);
    }
    public static bool operator !=(KeyTuple<TKey>? left, KeyTuple<TKey>? right)
    {
        if (ReferenceEquals(left, right)) return false;
        if (left is null || right is null) return true;
        return !EqualityComparer<TKey>.Default.Equals(left.Key, right.Key);
    }
    public static bool operator ==(KeyTuple<TKey>? keyTuple, TKey? key)
    {
        if (keyTuple is null) return false;
        return EqualityComparer<TKey>.Default.Equals(keyTuple.Key, key);
    }
    public static bool operator !=(KeyTuple<TKey>? keyTuple, TKey? key)
    {
        if (keyTuple is null) return true;
        return !EqualityComparer<TKey>.Default.Equals(keyTuple.Key, key);
    }
    public static bool operator >(KeyTuple<TKey> left, KeyTuple<TKey> right) 
        => Comparer<TKey>.Default.Compare(left.Key, right.Key) > 0;
    public static bool operator >=(KeyTuple<TKey> left, KeyTuple<TKey> right)
        => Comparer<TKey>.Default.Compare(left.Key, right.Key) >= 0;
    public static bool operator <(KeyTuple<TKey> left, KeyTuple<TKey> right)
        => Comparer<TKey>.Default.Compare(left.Key, right.Key) < 0;
    public static bool operator <=(KeyTuple<TKey> left, KeyTuple<TKey> right)
        => Comparer<TKey>.Default.Compare(left.Key, right.Key) <= 0;
    public static bool operator >(KeyTuple<TKey> left, TKey right)
        => Comparer<TKey>.Default.Compare(left.Key, right) > 0;
    public static bool operator >=(KeyTuple<TKey> left, TKey right)
        => Comparer<TKey>.Default.Compare(left.Key, right) >= 0;
    public static bool operator <(KeyTuple<TKey> left, TKey right)
        => Comparer<TKey>.Default.Compare(left.Key, right) < 0;
    public static bool operator <=(KeyTuple<TKey> left, TKey right)
        => Comparer<TKey>.Default.Compare(left.Key, right) <= 0;

    public TKey Key { get; }

    public KeyTuple(TKey key)
    {
        this.Key = key;
    }

    public int CompareTo(TKey? key)
    {
        return Comparer<TKey>.Default.Compare(this.Key, key);
    }
    
    public int CompareTo(KeyTuple<TKey>? keyTuple)
    {
        if (keyTuple is null) return 1; // null always sorts first
        return Comparer<TKey>.Default.Compare(this.Key, keyTuple.Key);
    }

    public bool Equals(TKey? key)
    {
        return EqualityComparer<TKey>.Default.Equals(this.Key, key);
    }
    
    public bool Equals(KeyTuple<TKey>? keyTuple)
    {
        if (keyTuple is null) return false;
        return EqualityComparer<TKey>.Default.Equals(this.Key, keyTuple.Key);
    }

    public override bool Equals(object? obj)
    {
        if (obj is KeyTuple<TKey> keyTuple) return EqualityComparer<TKey>.Default.Equals(this.Key, keyTuple.Key);
        if (obj is TKey key) return EqualityComparer<TKey>.Default.Equals(this.Key, key);
        return false;
    }
    public override int GetHashCode()
    {
        return this.Key?.GetHashCode() ?? 0;
    }
    public override string ToString()
    {
        return $"({Key})";
    }
}