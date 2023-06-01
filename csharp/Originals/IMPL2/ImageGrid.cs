namespace AllColors.Originals.IMPL2;

public sealed class ImageGrid
{
    private readonly ImageGridCell[,] _cells;
    
    public int Width { get; }
    public int Height { get; }
    
    public ImageGridCell this[int x, int y] => _cells[x, y];
    public ImageGridCell this[Point pt] => _cells[pt.X, pt.Y];

    public ImageGrid(Size size)
        : this(size.Width, size.Height)
    {
        
    }
    
    public ImageGrid(int width, int height)
    {
        this.Width = width;
        this.Height = height;
        _cells = new ImageGridCell[width, height];
        for (var y = 0; y < height; y++)
        {
            for (var x = 0; x < width; x++)
            {
                _cells[x, y] = new ImageGridCell(x, y);
            }
        }
        foreach (var cell in _cells)
        {
            cell.Initialize(this);
        }
    }
}