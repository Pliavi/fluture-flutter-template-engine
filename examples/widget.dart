import 'package:flutter/material.dart';

class ProfileTile extends StatefulWidget {
  final String name;
  final String email;

  const ProfileTile({
    super.key,
    this.name,
    this.email,
  });
}

class ProfileTileState extends State<ProfileTile> {
  num counter = 12;

  @override
  Widget build(BuildContext context) {
    final Profile profile = context.watch<Profile>();
    final Api api = context.read<Api>();
    final Customer customer = context.select(
      (CustomerProvider p) => Customer(name: p.name),
    );

    return Container(
      child: ListTile(
        title: Text(widget.name),
        subtitle: Text(widget.email),
        trailing: Text('$counter'),
      ),
    );
  }
}
