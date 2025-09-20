// TODO: Develop the "train" CLI subcommand: it calculates the frequencies of all the words encountered in the text and calculates easiness ratings of all the sentences.
// TODO: develop "show frequencies" subcommand: it displays a sorted list of `word: <number-of-times-encountered>`
// TODO: Develop "show sentences" subcommand: it displays a sorted list of `<place>. `<sentence>`: <easiness-rating>`

using Spectre.Console.Cli;

public static class Program
{
    public static int Main(string[] args)
    {
        var app = new CommandApp();
        app.Configure(cfg =>
        {
            cfg.SetApplicationName("freq-sage");
            cfg.AddCommand<TrainCommand>("train")
                .WithDescription("Trains the database on the specified text file. Calculates word frequencies and sentence easiness ratings for this file and adds them to the database.");
        });

        return app.Run(args);
    }
}
