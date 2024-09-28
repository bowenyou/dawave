pub mod common {
    tonic::include_proto!("common");
}

pub mod churner {
    tonic::include_proto!("churner");
}

pub mod retriever {
    tonic::include_proto!("retriever");
}

pub mod node {
    tonic::include_proto!("node");
}

pub mod disperser {
    tonic::include_proto!("disperser");
}
