import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestParam;
import org.springframework.web.bind.annotation.RestController;
import java.time.Duration;
import java.time.Instant;

// Define a class `PaymentParams` that represents the payment parameters
class PaymentParams {
    public Long id;
    public Double amount;
    public String payment_info;
}

// Define a class `PaymentResult` that represents the result of the payment
class PaymentResult {
    public Long id;
    public Double amount;
    public String payment_info;
    public Boolean success;
    public Boolean fraud;
    public Double duration;
    public String response;

    public PaymentResult(Long id, Double amount, String payment_info, Boolean success, Boolean fraud, Double duration, String response) {
        this.id = id;
        this.amount = amount;
        this.payment_info = payment_info;
        this.success = success;
        this.fraud = fraud;
        this.duration = duration;
        this.response = response;
    }

    @Override
    public String toString() {
        return String.format("| %4d | %20s | %20s | %10.2f$ | %10.2fns | %-10s | %-20s |",
                id,
                success ? "✅ Payment successful  " : "❌ Payment failed      ",
                fraud ? "🔔 Fraud detected      " : "👍 No fraud detected   ",
                amount,
                duration,
                payment_info,
                response);
    }
}

// Define a class `PaymentController` that processes the payment
@RestController
class PaymentController {

    @GetMapping("/payment")
    public PaymentResult processPayment(@RequestParam Long id, @RequestParam Double amount, @RequestParam String payment_info) {
        // Create a `PaymentParams` object from the query parameters
        PaymentParams paymentParams = new PaymentParams();
        paymentParams.id = id;
        paymentParams.amount = amount;
        paymentParams.payment_info = payment_info;

        // Get the start time
        Instant start = Instant.now();

        // Predict whether the payment is fraudulent based on the payment amount
        Boolean is_fraud = predictFraud(paymentParams);

        // Get the duration of the payment processing
        Double duration = (double) Duration.between(start, Instant.now()).toNanos();

        // Print the start time and duration to the console
        System.out.println("start: " + start);
        System.out.println("duration: " + duration);

        // Create a `PaymentResult` object from the payment processing results
        PaymentResult paymentResult = new PaymentResult(
                paymentParams.id,
                paymentParams.amount,
                paymentParams.payment_info,
                !is_fraud,
                is_fraud,
                duration,
                String.format("Payment %d processed", paymentParams.id));

        return paymentResult;
    }

    // Define the `predictFraud` function that predicts whether the payment is fraudulent
    // This function takes in a `PaymentParams` object
    // If the payment amount is greater than 1000.0, it is considered fraudulent
    private Boolean predictFraud(PaymentParams paymentParams) {
        return paymentParams.amount > 1000.0;
    }
}

// Define the main function that starts the SpringBoot application
@SpringBootApplication
public class PaymentApplication {
    public static void main(String[] args) {
        SpringApplication.run(PaymentApplication.class, args);
    }
}
