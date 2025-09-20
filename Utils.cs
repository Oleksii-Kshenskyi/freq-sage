using System.Text;

public static class Utils {
    // REFACTOR: should return a Result<string, ErrorType> instead of printing errors to console, catching exceptions is not its responsibility
    public static string TryFileToText(string filename) {
        string text = "";
        try
        {
            text = File.ReadAllText(filename, Encoding.UTF8);
        }
        catch (FileNotFoundException)
        {
            Console.WriteLine($"ERROR: file `{filename}` does not exist!");
        }
        catch (UnauthorizedAccessException)
        {
            Console.WriteLine($"ERROR: no permission to read file `{filename}`!");
        }
        catch (DirectoryNotFoundException) { 
            Console.WriteLine($"ERROR: directory of the file `{filename}` does not exist!");
        }
        catch (IOException e) {
            Console.WriteLine($"ERROR: I/O error while reading file `{filename}`: {e.Message}");
        }

        return text;
    }

    // TODO: CleanWord doesn't do anything - needs to clean words based on a collection of regexes in its argument
    // TODO [followup]: implement two arrays of regexes as constants: one for words and the otehr for sentences
    public static string CleanWord(string word) {
        return word;
    }
}