use ferrite_session::*;

/*
 type AuthenticatedPubSub =
   \lineartoshared
     &{
       Password:
         Credentials ⊳
         ⊕{
           Success:
             PubSub x
             \sharedtolinear
               AuthenticatedPubSub,
           Failure:
             AuthError ⊲
             \sharedtolinear
               AuthenticatedPubSub
         },
       Challenge:
         String ⊲
         String ⊳
         ⊕{
           Success:
             PubSub x
             \sharedtolinear
               AuthenticatedPubSub,
           Failure:
             AuthError ⊲
             \sharedtolinear
               AuthenticatedPubSub
         }
     };

 type PubSub =
   &{
     Publish: Publisher,
     Subscribe: Subscriber
   };

 type Publisher =
   &{
     SendMessage:
       String ⊳ Publisher,
     StopPublish:
       ε
   };

 type Subscriber =
   &{
     ReceiveMessage:
       ⊕{
         Message:
           String ⊲ Subscriber,
         Ended:
           ε
       },
     StopSubscribe:
       ε
   };
*/

pub struct Credentials
{
  pub username : String,
  pub password : String,
}

pub struct AuthError
{
  pub reason : String,
}

type AuthenticatedPubSub = LinearToShared<ExternalChoice<AuthChoice>>;

define_choice! { AuthChoice;
  Password:
    SendValue <
      Credentials,
      InternalChoice < AuthResult >
    >,
  Challenge:
    ReceiveValue <
      String,
      SendValue <
        String,
        InternalChoice < AuthResult >
      >
    >
}

define_choice! { AuthResult;
  Success:
    SendChannel <
      PubSub,
      Z // AuthenticatedPubSub
    >,
  Failure:
    SendValue <
      AuthError,
      Z // AuthenticatedPubSub
    >,
}

type PubSub = ExternalChoice<PubSubChoice>;

define_choice! { PubSubChoice;
  Publish: Publisher,
  Subscribe: Subscription
}

type Publisher = Rec<ExternalChoice<PublishAction>>;

define_choice! { PublishAction;
  SendMessage:
    SendValue <
      String,
      Z // Publisher
    >,
  StopPublish: End
}

type Subscription = Rec<ExternalChoice<SubscribeAction>>;

define_choice! { SubscribeAction;
  Poll: InternalChoice < SubscribeEvent >,
  StopSubscribe: End
}

define_choice! { SubscribeEvent;
  Timeout: Z, // Subscription
  MessageEvent: ReceiveValue < String, End >,
  Ended: End,
}

pub fn create_authenticated_pub_sub() -> SharedSession<AuthenticatedPubSub>
{
  todo!()
}

#[tokio::main]

pub async fn main() {}
