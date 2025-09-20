using System.Runtime.InteropServices;

class Frequencies
{
    public Dictionary<string, ulong> Map { get; }

    public Frequencies(string text)
    {
        Map = FreqsFromText(text);
    }

    private Dictionary<string, ulong> FreqsFromText(string text)
    {
        var words = text.Split([' ', '\n', '\r', '\t'], StringSplitOptions.RemoveEmptyEntries);

        var freq_map = new Dictionary<string, ulong>();
        foreach (var word in words) {
            var clean_word = Utils.CleanWord(word);
            ref var count = ref CollectionsMarshal.GetValueRefOrAddDefault(freq_map, clean_word, out var exists);
            count = exists ? count + 1 : 1;
        }

        return freq_map;
    }
}

