using Spectre.Console.Cli;
using System.ComponentModel;

public sealed class TrainSettings: CommandSettings
{
    [CommandArgument(0, "<text>")]
    [Description("Text to train the database on.")]
    public string TextPath { get; init; } = "";
}

public sealed class TrainCommand : Command<TrainSettings> {
    public override int Execute(CommandContext context, TrainSettings s) {
        var text = Utils.TryFileToText(s.TextPath);
        var freqs = new Frequencies(text);
        foreach (var (word, count) in freqs.Map)
        { }
        return 0;
    }
}

