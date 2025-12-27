using Microsoft.ApplicationInsights;
using Microsoft.ApplicationInsights.DataContracts;

namespace Arxis.API.Services;

public interface IAnalyticsService
{
    // User Events
    void TrackUserSignup(string userId, string email, string plan = "free");
    void TrackUserLogin(string userId);
    void TrackUserLogout(string userId);

    // Business Events
    void TrackPlanInterest(string userId, string planName, decimal planPrice);
    void TrackUpgradeIntent(string userId, string fromPlan, string toPlan);
    void TrackCheckoutStarted(string userId, string planName, decimal amount);
    void TrackCheckoutCompleted(string userId, string planName, decimal amount, string paymentMethod);
    void TrackCheckoutAbandoned(string userId, string planName, decimal amount, string reason);

    // Feature Usage
    void TrackFeatureUsed(string userId, string featureName, Dictionary<string, string>? properties = null);
    void TrackProjectCreated(string userId, string projectId, string projectType);
    void TrackIssueCreated(string userId, string issueId, string priority);
    void TrackDocumentUploaded(string userId, string documentId, long fileSize, string fileType);

    // Email Events
    void TrackEmailSent(string to, string template, bool success, string? errorMessage = null);
    void TrackEmailOpened(string to, string template);
    void TrackEmailClicked(string to, string template, string linkUrl);

    // Performance
    void TrackApiRequest(string endpoint, int statusCode, long durationMs);
    void TrackException(Exception exception, Dictionary<string, string>? properties = null);

    // Conversion Funnel
    void TrackFunnelStep(string userId, string funnelName, string stepName, Dictionary<string, string>? properties = null);
}

public class AnalyticsService : IAnalyticsService
{
    private readonly TelemetryClient _telemetry;
    private readonly ILogger<AnalyticsService> _logger;

    public AnalyticsService(TelemetryClient telemetry, ILogger<AnalyticsService> logger)
    {
        _telemetry = telemetry;
        _logger = logger;
    }

    public void TrackUserSignup(string userId, string email, string plan = "free")
    {
        var properties = new Dictionary<string, string>
        {
            { "userId", userId },
            { "email", email },
            { "plan", plan },
            { "signupDate", DateTime.UtcNow.ToString("O") }
        };

        _telemetry.TrackEvent("UserSignup", properties);
        _logger.LogInformation("User signup tracked: {UserId}, Plan: {Plan}", userId, plan);
    }

    public void TrackUserLogin(string userId)
    {
        _telemetry.TrackEvent("UserLogin", new Dictionary<string, string>
        {
            { "userId", userId },
            { "loginTime", DateTime.UtcNow.ToString("O") }
        });
    }

    public void TrackUserLogout(string userId)
    {
        _telemetry.TrackEvent("UserLogout", new Dictionary<string, string>
        {
            { "userId", userId }
        });
    }

    public void TrackPlanInterest(string userId, string planName, decimal planPrice)
    {
        var properties = new Dictionary<string, string>
        {
            { "userId", userId },
            { "planName", planName },
            { "planPrice", planPrice.ToString("F2") },
            { "timestamp", DateTime.UtcNow.ToString("O") }
        };

        _telemetry.TrackEvent("PlanInterest", properties);
        _logger.LogInformation("Plan interest tracked: User {UserId} interested in {PlanName} (${Price})",
            userId, planName, planPrice);
    }

    public void TrackUpgradeIntent(string userId, string fromPlan, string toPlan)
    {
        _telemetry.TrackEvent("UpgradeIntent", new Dictionary<string, string>
        {
            { "userId", userId },
            { "fromPlan", fromPlan },
            { "toPlan", toPlan },
            { "timestamp", DateTime.UtcNow.ToString("O") }
        });

        _logger.LogInformation("Upgrade intent: User {UserId} from {FromPlan} to {ToPlan}",
            userId, fromPlan, toPlan);
    }

    public void TrackCheckoutStarted(string userId, string planName, decimal amount)
    {
        var properties = new Dictionary<string, string>
        {
            { "userId", userId },
            { "planName", planName },
            { "amount", amount.ToString("F2") },
            { "currency", "USD" }
        };

        _telemetry.TrackEvent("CheckoutStarted", properties, new Dictionary<string, double>
        {
            { "revenue", (double)amount }
        });

        _logger.LogInformation("🛒 Checkout started: User {UserId}, Plan {PlanName}, ${Amount}",
            userId, planName, amount);
    }

    public void TrackCheckoutCompleted(string userId, string planName, decimal amount, string paymentMethod)
    {
        var properties = new Dictionary<string, string>
        {
            { "userId", userId },
            { "planName", planName },
            { "amount", amount.ToString("F2") },
            { "currency", "USD" },
            { "paymentMethod", paymentMethod }
        };

        _telemetry.TrackEvent("CheckoutCompleted", properties, new Dictionary<string, double>
        {
            { "revenue", (double)amount }
        });

        // Track as purchase/conversion
        _telemetry.TrackEvent("Purchase", properties, new Dictionary<string, double>
        {
            { "revenue", (double)amount }
        });

        _logger.LogInformation("💰 VENDA! User {UserId} comprou {PlanName} por ${Amount} via {PaymentMethod}",
            userId, planName, amount, paymentMethod);
    }

    public void TrackCheckoutAbandoned(string userId, string planName, decimal amount, string reason)
    {
        _telemetry.TrackEvent("CheckoutAbandoned", new Dictionary<string, string>
        {
            { "userId", userId },
            { "planName", planName },
            { "amount", amount.ToString("F2") },
            { "reason", reason },
            { "timestamp", DateTime.UtcNow.ToString("O") }
        });

        _logger.LogWarning("❌ Checkout abandonado: User {UserId}, Plan {PlanName}, Razão: {Reason}",
            userId, planName, reason);
    }

    public void TrackFeatureUsed(string userId, string featureName, Dictionary<string, string>? properties = null)
    {
        var props = properties ?? new Dictionary<string, string>();
        props["userId"] = userId;
        props["featureName"] = featureName;
        props["timestamp"] = DateTime.UtcNow.ToString("O");

        _telemetry.TrackEvent("FeatureUsed", props);
    }

    public void TrackProjectCreated(string userId, string projectId, string projectType)
    {
        _telemetry.TrackEvent("ProjectCreated", new Dictionary<string, string>
        {
            { "userId", userId },
            { "projectId", projectId },
            { "projectType", projectType }
        });
    }

    public void TrackIssueCreated(string userId, string issueId, string priority)
    {
        _telemetry.TrackEvent("IssueCreated", new Dictionary<string, string>
        {
            { "userId", userId },
            { "issueId", issueId },
            { "priority", priority }
        });
    }

    public void TrackDocumentUploaded(string userId, string documentId, long fileSize, string fileType)
    {
        _telemetry.TrackEvent("DocumentUploaded", new Dictionary<string, string>
        {
            { "userId", userId },
            { "documentId", documentId },
            { "fileType", fileType }
        }, new Dictionary<string, double>
        {
            { "fileSizeBytes", fileSize }
        });
    }

    public void TrackEmailSent(string to, string template, bool success, string? errorMessage = null)
    {
        var properties = new Dictionary<string, string>
        {
            { "to", to },
            { "template", template },
            { "success", success.ToString() }
        };

        if (!success && errorMessage != null)
        {
            properties["error"] = errorMessage;
        }

        _telemetry.TrackEvent("EmailSent", properties);
    }

    public void TrackEmailOpened(string to, string template)
    {
        _telemetry.TrackEvent("EmailOpened", new Dictionary<string, string>
        {
            { "to", to },
            { "template", template },
            { "openedAt", DateTime.UtcNow.ToString("O") }
        });
    }

    public void TrackEmailClicked(string to, string template, string linkUrl)
    {
        _telemetry.TrackEvent("EmailClicked", new Dictionary<string, string>
        {
            { "to", to },
            { "template", template },
            { "linkUrl", linkUrl }
        });
    }

    public void TrackApiRequest(string endpoint, int statusCode, long durationMs)
    {
        var request = new RequestTelemetry
        {
            Name = endpoint,
            ResponseCode = statusCode.ToString(),
            Duration = TimeSpan.FromMilliseconds(durationMs),
            Success = statusCode >= 200 && statusCode < 400
        };

        _telemetry.TrackRequest(request);
    }

    public void TrackException(Exception exception, Dictionary<string, string>? properties = null)
    {
        _telemetry.TrackException(exception, properties);
        _logger.LogError(exception, "Exception tracked in analytics");
    }

    public void TrackFunnelStep(string userId, string funnelName, string stepName, Dictionary<string, string>? properties = null)
    {
        var props = properties ?? new Dictionary<string, string>();
        props["userId"] = userId;
        props["funnelName"] = funnelName;
        props["stepName"] = stepName;
        props["timestamp"] = DateTime.UtcNow.ToString("O");

        _telemetry.TrackEvent($"Funnel_{funnelName}_{stepName}", props);

        _logger.LogInformation("Funnel step: {UserId} → {FunnelName} → {StepName}",
            userId, funnelName, stepName);
    }
}
