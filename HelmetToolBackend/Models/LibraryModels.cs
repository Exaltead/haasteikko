using HelmetToolBackend.Shared;

namespace HelmetToolBackend.Models
{
    public record LibraryBook
    {
        public string Title { get; set; } = string.Empty;
        public string Author { get; set; } = string.Empty;
        public string? Translator { get; set; }
    }

    public record LibraryItem : IDbEntity
    {
        public LibraryBook? Book { get; set; }
        public string Id { get; set; } = "";

        public string? UserId { get; set; }
        public DateTimeOffset AddDate { get; set; } = DateTimeOffset.UtcNow;

        public List<string> ActivatedChallengeIds { get; set; } = [];

    }
}