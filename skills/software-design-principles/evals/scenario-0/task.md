# Code Review: E-Commerce Order Service

## Problem/Feature Description

You are conducting a design review of an e-commerce platform's order processing service. The team has been experiencing maintainability issues - developers frequently conflict when merging changes, tests are difficult to write, and adding new payment methods requires modifying existing stable code.

The codebase has evolved organically over 18 months and now contains several design issues that need to be identified and documented. Your goal is to perform a thorough design review, identify the root causes of the maintainability problems, and provide actionable recommendations.

## Output Specification

Create a design review report called `design-review.md` that:

1. **Classifies** what type of design decision this review involves (architectural, tactical, or foundational)
2. **Applies SOLID principles** - analyze each class/interaction for violations of Single Responsibility, Open-Closed, Liskov Substitution, Interface Segregation, and Dependency Inversion principles
3. **Identifies anti-patterns** - document any god classes, circular dependencies, hardcoded secrets, or direct concrete coupling you find
4. **Provides recommendations** using BAD/GOOD format showing problematic code and the recommended fix
5. **Documents tradeoffs** - for your key recommendations, explain alternatives considered, why you chose the recommended approach, and potential risks

The review should be comprehensive enough that another developer could use it as a guide for refactoring.

## Input Files (optional)

The following files are provided as inputs. Extract them before beginning.

=============== FILE: order-service.ts ===============
export class OrderService {
  private db: Postgres private emailClientDatabase;
 : SendGridClient;
  private stripe: StripeGateway;
  private logger: WinstonLogger;

  constructor() {
    this.db = new PostgresDatabase(process.env.DB_URL);
    this.emailClient = new SendGridClient(process.env.SENDGRID_API_KEY);
    this.stripe = new StripeGateway(process.env.STRIPE_KEY);
    this.logger = new WinstonLogger();
  }

  async createOrder(orderData: any): Promise&lt;Order&gt; {
    // Validate order data
    if (!orderData.items || orderData.items.length === 0) {
      throw new Error('Order must have items');
    }

    // Calculate total
    let total = 0;
    for (const item of orderData.items) {
      total += item.price * item.quantity;
    }

    // Apply discounts
    if (orderData.couponCode) {
      const discount = await this.getDiscount(orderData.couponCode);
      total = total * (1 - discount.percentage);
    }

    // Save to database
    const order = await this.db.orders.insert({
      ...orderData,
      total,
      status: 'pending',
      createdAt: new Date(),
    });

    // Send confirmation email
    const user = await this.db.users.findById(orderData.userId);
    await this.emailClient.send({
      to: user.email,
      template: 'order-confirmation',
      data: { order, user },
    });

    // Process payment
    await this.stripe.charge(user.stripeCustomerId, total);

    // Update order status
    order.status = 'confirmed';
    await this.db.orders.update(order.id, { status: 'confirmed' });

    // Log the transaction
    this.logger.info('Order created', { orderId: order.id, total });

    // Notify inventory
    await this.db.inventory.reserve(orderData.items);

    // Send shipping notification after 24 hours
    // TODO: Implement scheduled notification
    if (orderData.shippingAddress) {
      // Future: Integrate with shipping providers
      // Future: Calculate shipping rates based on address
      // Future: Support multiple carriers
    }

    return order;
  }

  async getDiscount(code: string): Promise&lt;Discount&gt; {
    return await this.db.discounts.findByCode(code);
  }

  async cancelOrder(orderId: string, reason: string): Promise&lt;void&gt; {
    const order = await this.db.orders.findById(orderId);
    if (!order) {
      throw new Error('Order not found');
    }

    if (order.status === 'shipped') {
      throw new Error('Cannot cancel shipped orders');
    }

    // Refund payment
    await this.stripe.refund(order.chargeId);

    // Send cancellation email
    const user = await this.db.users.findById(order.userId);
    await this.emailClient.send({
      to: user.email,
      template: 'order-cancelled',
      data: { order, reason },
    });

    // Update inventory
    await this.db.inventory.release(order.items);

    // Update status
    await this.db.orders.update(orderId, { status: 'cancelled', cancellationReason: reason });

    this.logger.info('Order cancelled', { orderId, reason });
  }

  async getOrderHistory(userId: string): Promise<Order[]> {
    return await this.db.orders.findByUserId(userId);
  }

  async generateReport(startDate: Date, endDate: Date): Promise&lt;any&gt; {
    const orders = await this.db.orders.findByDateRange(startDate, endDate);

    let revenue = 0;
    let orderCount = 0;
    let cancelledCount = 0;

    for (const order of orders) {
      if (order.status !== 'cancelled') {
        revenue += order.total;
        orderCount++;
      } else {
        cancelledCount++;
      }
    }

    return {
      revenue,
      orderCount,
      cancelledCount,
      averageOrderValue: orderCount > 0 ? revenue / orderCount : 0,
    };
  }
}

=============== FILE: payment-gateway.ts ===============
export class StripeGateway {
  private apiKey: string;

  constructor(apiKey: string) {
    this.apiKey = apiKey;
  }

  async charge(customerId: string, amount: number): Promise&lt;ChargeResult&gt; {
    // Direct Stripe API calls - no abstraction
    const stripe = require('stripe')(this.apiKey);
    const charge = await stripe.charges.create({
      amount: amount * 100,
      currency: 'usd',
      customer: customerId,
    });
    return { id: charge.id, status: charge.status };
  }

  async refund(chargeId: string): Promise&lt;RefundResult&gt; {
    const stripe = require('stripe')(this.apiKey);
    const refund = await stripe.refunds.create({ charge: chargeId });
    return { id: refund.id, status: refund.status };
  }
}

=============== FILE: database.ts ===============
export class PostgresDatabase {
  private connectionString: string;

  constructor(connectionString: string) {
    this.connectionString = connectionString;
  }

  get orders() {
    return {
      insert: async (data: any) => ({ id: 'order-1', ...data }),
      findById: async (id: string) => ({ id, status: 'pending' }),
      findByUserId: async (userId: string) => [],
      findByDateRange: async (start: Date, end: Date) => [],
      update: async (id: string, data: any) => {},
    };
  }

  get users() {
    return {
      findById: async (id: string) => ({ id, email: `user@example.com`, stripeCustomerId: 'cus_123' }),
    };
  }

  get discounts() {
    return {
      findByCode: async (code: string) => ({ percentage: 0.1 }),
    };
  }

  get inventory() {
    return {
      reserve: async (items: any[]) => {},
      release: async (items: any[]) => {},
    };
  }
}
