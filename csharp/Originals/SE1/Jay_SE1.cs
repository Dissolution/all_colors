namespace AllColors.Originals.SE1;

public class Jay_SE1
{
    // gets the difference between two colors
    private static int ColorDiff(Color left, Color right)
    {
        var r = left.R - right.R;
        var g = left.G - right.G;
        var b = left.B - right.B;
        return (r * r) + (g * g) + (b * b);
    }
    
    
    // algorithm settings, feel free to mess with it
    private const bool USE_AVERAGE = false;

    private readonly ColorSpace _colorSpace;
    private readonly Shuffler _shuffler;
    
    public Jay_SE1(ColorSpace colorSpace, Shuffler shuffler)
    {
        _colorSpace = colorSpace;
        _shuffler = shuffler;
    }

    // gets the neighbors (3..8) of the given coordinate
    private List<Point> GetNeighbors2(Point xy)
    {
        var neighbors = new List<Point>(9); // 8
        for (var deltaY = -1; deltaY <= 1; deltaY++)
        {
            if (xy.Y + deltaY == -1 || xy.Y + deltaY == _colorSpace.Height)
                continue;
            for (var deltaX = -1; deltaX <= 1; deltaX++)
            {
                if (xy.X + deltaX == -1 || xy.X + deltaX == _colorSpace.Width)
                    continue;
                // Skip me!
                //if (deltaY == 0 && deltaX == 0) continue;
                
                neighbors.Add(new Point(xy.X + deltaX, xy.Y + deltaY));
            }
        }
        if (neighbors.Count > 9) // 8
            Debugger.Break();
        return neighbors;
    }

    private List<Point> GetNeighbors(Point point)
    {
        var neighbors = new List<Point>(9);
        for (var dy = -1; dy <= 1; dy++)
        {
            var ny = point.Y + dy;
            if (ny == -1 || ny == _colorSpace.Height) continue;
            for (var dx = -1; dx <= 1; dx++)
            {
                if (dx == 0 && dy == 0) continue;
                
                var nx = point.X + dx;
                if (nx == -1 || nx == _colorSpace.Width) continue;

                neighbors.Add(new Point(nx, ny));
            }
        }

        // var n2 = GetNeighbors2(point);
        // var eq = neighbors.SequenceEqual(n2);
        // Debug.Assert(eq);
        return neighbors;
    }
    
    // calculates how well a color fits at the given coordinates
    private int CalcDiff(Color[,] pixels, Point xy, Color color)
    {
        // get the diffs for each neighbor separately
        var diffs = new List<int>(8);
        foreach (var nxy in GetNeighbors(xy))
        {
            var neighborColor = pixels[nxy.X, nxy.Y];
            if (!neighborColor.IsEmpty)
            {
                diffs.Add(ColorDiff(neighborColor, color));
            }
        }
        if (diffs.Count > 8)
            Debugger.Break();

        // average or minimum selection
        if (USE_AVERAGE)
            return (int)diffs.Average();
        else
            return diffs.Min();
    }
    
    public Bitmap CreateImage()
    {
        var (_, width, height) = _colorSpace;

        var colors = _colorSpace.GetColors().OrderBy(_ => _shuffler.ZeroTo(3) - 1).ToArray();
        //var colors = _colorSpace.GetColors();
        //_shuffler.Shuffle<Color>(colors);
        
        
        // temporary place where we work (faster than all that many GetPixel calls)
        var pixelColors = new Color[width, height];
        Trace.Assert(pixelColors.Length == colors.Length);

        // constantly changing list of available coordinates
        // (empty pixels which have non-empty neighbors)
        var available = new HashSet<Point>(pixelColors.Length / 8); // guess
        
        // loop through all colors that we want to place
        for (var i = 0; i < colors.Length; i++)
        {
            if (i % 256 == 0)
            {
                Console.WriteLine("{0:P}, queue size {1}", (double)i / width / height, available.Count);
            }

            Point bestPoint;
            if (available.Count == 0)
            {
                // use the starting point
                bestPoint = _colorSpace.MidPoint;
            }
            else
            {
                // find the best place from the list of available coordinates
                // uses parallel processing, this is the most expensive step
                bestPoint = available
                    .AsParallel()
                    .OrderBy(xy => CalcDiff(pixelColors, xy, colors[i]))
                    .First();
            }

            // put the pixel where it belongs
            Trace.Assert(pixelColors[bestPoint.X, bestPoint.Y].IsEmpty);
            pixelColors[bestPoint.X, bestPoint.Y] = colors[i];

            // adjust the available list
            available.Remove(bestPoint);
            //Trace.Assert(removed);
            foreach (var nxy in GetNeighbors(bestPoint))
            {
                if (pixelColors[nxy.X, nxy.Y].IsEmpty)
                {
                    available.Add(nxy);
                }
            }
        }

        Trace.Assert(available.Count == 0);
        
        var image = new Bitmap(width, height, PixelFormat.Format24bppRgb);
        for (var y = 0; y < height; y++)
        {
            for (var x = 0; x < width; x++)
            {
                image.SetPixel(x, y, pixelColors[x, y]);
            }
        }
        return image;
    }
}